use crate::audio::cpal_stream::create_audio_stream;
use crate::audio::decoding::{decoder_thread, probe_audio_file_format};
use crate::audio::position_updater::position_updater_thread;
use crate::audio::shared::{AudioCommand, DecoderCommand, PlaybackState};
use cpal::traits::StreamTrait;
use cpal::Stream;
use ringbuf::traits::Split;
use ringbuf::HeapRb;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::AppHandle;

pub fn audio_thread(receiver: Receiver<AudioCommand>, app_handle: AppHandle) {
    let state = Arc::new(Mutex::new(PlaybackState {
        is_playing: false,
        is_paused: false,
        volume: 0.5,
        current_position_samples: 0,
        sample_rate: 48000,
        duration_samples: None,
        needs_buffer_clear: false,
    }));

    // spawn position updater thread
    let state_clone = state.clone();
    let app_handle_clone = app_handle.clone();
    thread::spawn(move || {
        position_updater_thread(state_clone, app_handle_clone);
    });

    let mut decoder_handle: Option<thread::JoinHandle<_>> = None;
    let mut decoder_command_sender: Option<Sender<DecoderCommand>> = None;
    let mut stream: Option<Stream> = None;

    // command loop
    loop {
        match receiver.recv() {
            Ok(AudioCommand::LoadAndPlay(path)) => {
                println!("Loading: {}", path);

                // stop existing playback
                {
                    let mut state = state.lock().unwrap();
                    state.is_playing = false;
                }

                // send stop command to old decoder
                if let Some(decoder_command_sender) = decoder_command_sender.take() {
                    let _ = decoder_command_sender.send(DecoderCommand::Stop);
                }

                // Let old decoder finish in background - don't block!
                if let Some(handle) = decoder_handle.take() {
                    thread::spawn(move || {
                        let _ = handle.join(); // Clean up in background
                    });
                }

                // Drop old stream immediately
                drop(stream.take());

                // Probe the file to get sample rate and format
                let probe_result = match probe_audio_file_format(&path) {
                    Ok(pr) => pr,
                    Err(e) => {
                        eprintln!("Failed to probe audio file: {}", e);
                        continue; // Skip this song, don't crash
                    }
                };

                // Extract sample rate and channel count from probe result
                let (sample_rate, channels) = match probe_result.format.default_track() {
                    Some(track) => {
                        let sr = match track.codec_params.sample_rate {
                            Some(sr) => sr,
                            None => {
                                eprintln!("No sample rate found in audio file");
                                continue;
                            }
                        };
                        let ch = track.codec_params.channels.map(|c| c.count()).unwrap_or(2);
                        (sr, ch as u16)
                    }
                    None => {
                        eprintln!("No default track found");
                        continue;
                    }
                };

                println!("File sample rate: {}, channels: {}", sample_rate, channels);

                let ring_buffer = HeapRb::<f32>::new(sample_rate as usize * channels as usize);
                let (producer, consumer) = ring_buffer.split();

                // create audio output stream with the correct sample rate and channels
                let new_stream =
                    match create_audio_stream(state.clone(), consumer, sample_rate, channels) {
                        Ok(stream) => stream,
                        Err(e) => {
                            eprintln!("Failed to create audio output: {}", e);
                            return;
                        }
                    };

                // start the stream
                if let Err(e) = new_stream.play() {
                    eprintln!("Failed to start audio stream: {}", e);
                    return;
                }

                // keep stream from being dropped at end of loop
                stream = Some(new_stream);

                eprintln!("Audio output stream started");

                // create decoder command channel
                let (new_decoder_command_sender, decoder_command_receiver) =
                    channel::<DecoderCommand>();
                decoder_command_sender = Some(new_decoder_command_sender);

                // spawn new decoder thread with the probe result
                let state_clone = state.clone();

                decoder_handle = Some(thread::spawn(move || {
                    if let Err(e) = decoder_thread(
                        probe_result,
                        producer,
                        state_clone,
                        decoder_command_receiver,
                    ) {
                        eprintln!("Decoder error: {}", e);
                    }
                }));
            }

            Ok(AudioCommand::TogglePlayback) => {
                let mut state = state.lock().unwrap();
                state.is_paused = !state.is_paused;
                println!(
                    "Playback {}",
                    if state.is_paused {
                        "paused"
                    } else {
                        "resumed"
                    }
                );
            }
            Ok(AudioCommand::VolumeChange(volume)) => {
                let mut state = state.lock().unwrap();
                state.volume = volume;
                println!("Volume: {}", volume);
            }
            Ok(AudioCommand::Seek(position_seconds)) => {
                println!("Seeking to: {}s", position_seconds);
                if let Some(decoder_command_sender) = &decoder_command_sender {
                    let state = state.lock().unwrap();
                    let sample_rate = state.sample_rate;
                    drop(state);

                    let target_samples = (position_seconds * sample_rate as f64) as u64;

                    let _ = decoder_command_sender.send(DecoderCommand::Seek(target_samples));
                    println!(
                        "Seeking to {:.2} seconds ({} samples)",
                        position_seconds, target_samples
                    );
                }
            }
            Err(_) => {
                println!("Audio thread shutting down");
                break;
            }
        }
    }
    //cleanup
    {
        let mut state = state.lock().unwrap();
        state.is_playing = false;
    }

    // stop decoder
    if let Some(decoder_command_sender) = decoder_command_sender {
        let _ = decoder_command_sender.send(DecoderCommand::Stop);
    }

    if let Some(decoder_handle) = decoder_handle {
        let _ = decoder_handle.join();
    }
}
