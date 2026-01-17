use crate::audio::audio_thread::start_cpal_audio_stream;
use crate::player::probe::probe_audio_file;
use crate::player::shared::{PlaybackState};
use cpal::traits::StreamTrait;
use cpal::Stream;
use ringbuf::traits::Split;
use ringbuf::HeapRb;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use crate::decoder::decoder_commands::DecoderCommand;
use crate::decoder::decoder_thread::start_decoder_thread;

pub fn load_and_play(
    state: &Arc<Mutex<PlaybackState>>,
    decoder_handle: &mut Option<JoinHandle<()>>,
    decoder_command_sender: &mut Option<Sender<DecoderCommand>>,
    stream: &mut Option<Stream>,
    path: &String,
) -> () {
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

    // Let old decoder finish without blocking
    if let Some(handle) = decoder_handle.take() {
        thread::spawn(move || {
            let _ = handle.join();
        });
    }

    // Drop old stream immediately
    drop(stream.take());

    // Probe the file to get sample rate, channels and the format reader
    let probe_result = match probe_audio_file(&path) {
        Ok(pr) => pr,
        Err(e) => {
            eprintln!("Failed to probe audio file: {}", e);
            return;
        }
    };

    let format_reader = probe_result.format;
    let track = match format_reader.default_track() {
        Some(track) => track,
        None => {
            eprintln!("No default track found");
            return;
        }
    };

    let sample_rate = match track.codec_params.sample_rate {
        Some(sample_rate) => sample_rate,
        None => {
            eprintln!("No sample rate found in audio file");
            return;
        }
    };

    let channels = track
        .codec_params
        .channels
        .map(|c| c.count() as u16)
        .unwrap_or(2);

    println!("File sample rate: {}, channels: {}", sample_rate, channels);

    let sample_buffer = HeapRb::<f32>::new(sample_rate as usize * channels as usize);
    let (producer, consumer) = sample_buffer.split();

    // create audio output stream with the correct sample rate and channels
    let new_stream = match start_cpal_audio_stream(state.clone(), consumer, sample_rate, channels) {
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
    *stream = Some(new_stream);

    eprintln!("Audio output stream started");

    // create decoder command channel
    let (new_decoder_command_sender, decoder_command_receiver) = channel::<DecoderCommand>();

    *decoder_command_sender = Some(new_decoder_command_sender);

    // spawn new decoder thread
    let state_clone = state.clone();
    *decoder_handle = Some(thread::spawn(move || {
        if let Err(e) = start_decoder_thread(
            format_reader,
            producer,
            state_clone,
            decoder_command_receiver,
        ) {
            eprintln!("Decoder error: {}", e);
        }
    }));
}
