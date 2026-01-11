use crate::audio::load_and_play;
use crate::audio::position_updater::{EventEmitter, PositionUpdater, PositionUpdaterImpl};
use crate::audio::seek::seek;
use crate::audio::shared::{AudioPlayerCommand, DecoderCommand, PlaybackState};
use cpal::Stream;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tauri::AppHandle;

pub fn player_thread(receiver: Receiver<AudioPlayerCommand>, app_handle: AppHandle) {
    let state = Arc::new(Mutex::new(PlaybackState {
        is_playing: false,
        is_paused: false,
        volume: 0.5,
        current_position_samples: 0,
        sample_rate: 48000,
        needs_buffer_clear: false,
    }));

    // spawn position updater thread
    let state_clone = state.clone();
    let event_emitter: Arc<dyn EventEmitter + Send + Sync> = Arc::new(app_handle.clone());
    let position_updater = PositionUpdaterImpl::new(event_emitter);
    position_updater.start_thread(state_clone);

    let mut decoder_handle: Option<JoinHandle<_>> = None;
    let mut decoder_command_sender: Option<Sender<DecoderCommand>> = None;
    let mut stream: Option<Stream> = None;

    // command loop
    loop {
        match receiver.recv() {
            Ok(AudioPlayerCommand::LoadAndPlay(path)) => {
                load_and_play::load_and_play(
                    &state,
                    &mut decoder_handle,
                    &mut decoder_command_sender,
                    &mut stream,
                    &path,
                );
            }

            Ok(AudioPlayerCommand::TogglePlayback) => {
                toggle_playback(&state);
            }
            Ok(AudioPlayerCommand::VolumeChange(volume)) => {
                change_volume(&state, volume);
            }
            Ok(AudioPlayerCommand::Seek(position_seconds)) => {
                seek(&state, &mut decoder_command_sender, position_seconds);
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

fn toggle_playback(state: &Arc<Mutex<PlaybackState>>) {
    let mut state = state.lock().unwrap();
    state.is_paused = !state.is_paused;
    println!(
        "Playback {}",
        if state.is_paused { "paused" } else { "resumed" }
    );
}

fn change_volume(state: &Arc<Mutex<PlaybackState>>, volume: f32) {
    let mut state = state.lock().unwrap();
    state.volume = volume;
    println!("Volume: {}", volume);
}
