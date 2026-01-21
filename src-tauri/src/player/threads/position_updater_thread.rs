use crate::player::shared::PlaybackState;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub fn start_position_updater_thread(state: Arc<Mutex<PlaybackState>>, app_handle: Arc<AppHandle>) {
    thread::spawn(move || {
        position_updater_thread(state, app_handle);
    });
}

pub fn position_updater_thread(state: Arc<Mutex<PlaybackState>>, app_handle: Arc<AppHandle>) {
    loop {
        if let Some(audio_position) = get_audio_position(state.clone()) {
            _ = app_handle.emit("playback:position", audio_position);
        }
        thread::sleep(Duration::from_millis(40));
    }
}

fn get_audio_position(state: Arc<Mutex<PlaybackState>>) -> Option<AudioPosition> {
    let (position, is_playing) = {
        let state = state.lock().unwrap();
        (
            state.current_position_samples as f64 / state.sample_rate as f64,
            state.is_playing,
        )
    };

    if !is_playing {
        return None;
    }

    Some(AudioPosition {
        position_seconds: position,
    })
}

#[derive(serde::Serialize, Clone)]
pub struct AudioPosition {
    pub position_seconds: f64,
}

#[cfg(test)]
mod test {
    use super::*;

    // test if get_audio_position returns a valid and correctly calculated
    // audio position when is_playing is true
    #[test]
    fn test_get_audio_position_while_playing() {
        let state = Arc::new(Mutex::new(PlaybackState {
            is_playing: true,
            is_paused: false,
            volume: 0.5,
            current_position_samples: 48000,
            sample_rate: 48000,
            needs_buffer_clear: false,
        }));
        let result = get_audio_position(state.clone());
        assert!(result.is_some());
        assert_eq!(result.unwrap().position_seconds, 1f64)
    }

    // test if get_audio_position returns None if we are not playing
    #[test]
    fn test_get_audio_position_while_not_playing() {
        let state = Arc::new(Mutex::new(PlaybackState {
            is_playing: false,
            is_paused: false,
            volume: 0.5,
            current_position_samples: 48000,
            sample_rate: 48000,
            needs_buffer_clear: false,
        }));
        let result = get_audio_position(state.clone());
        assert!(result.is_none());
    }
}
