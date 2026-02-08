use crate::player::commands::change_volume::change_volume;
use crate::player::commands::load_and_play;
use crate::player::commands::seek::seek;
use crate::player::commands::toggle_playback::toggle_playback;
use crate::player::shared::{AudioPlayerCommand, PlaybackState};
use cpal::Stream;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tauri::AppHandle;
use crate::decoder::decoder_commands::DecoderCommand;
use crate::player::threads::position_updater_thread::start_position_updater_thread;

pub fn player_thread(
    receiver: Receiver<AudioPlayerCommand>,
    app_handle: Arc<AppHandle>,
) {
    let state = Arc::new(Mutex::new(PlaybackState {
        is_playing: false,
        is_paused: false,
        volume: 0.05,
        current_position_samples: 0,
        sample_rate: 48000,
        needs_buffer_clear: false,
    }));

    // spawn position updater thread
    let state_clone = state.clone();
    start_position_updater_thread(state_clone, app_handle);

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
