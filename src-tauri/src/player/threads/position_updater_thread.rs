use crate::player::shared::PlaybackState;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub trait EventEmitter {
    fn emit_position(&self, position: AudioPosition) -> ();
}

impl EventEmitter for AppHandle {
    fn emit_position(&self, position: AudioPosition) -> () {
        _ = self.emit("playback:position", position);
    }
}

pub trait PositionUpdater {
    fn new(event_emitter: Arc<dyn EventEmitter + Send + Sync>) -> Self;
    fn start_thread(&self, state: Arc<Mutex<PlaybackState>>);
}

pub struct PositionUpdaterImpl {
    event_emitter: Arc<dyn EventEmitter + Send + Sync>,
}
impl PositionUpdater for PositionUpdaterImpl {
    fn new(event_emitter: Arc<dyn EventEmitter + Send + Sync>) -> Self {
        Self { event_emitter }
    }

    fn start_thread(&self, state: Arc<Mutex<PlaybackState>>) {
        let event_emitter_clone = self.event_emitter.clone();
        thread::spawn(move || {
            position_updater_thread(state, event_emitter_clone);
        });
    }
}

pub fn position_updater_thread(
    state: Arc<Mutex<PlaybackState>>,
    event_emitter: Arc<dyn EventEmitter + Send + Sync>,
) {
    loop {
        if let Some(audio_position) = get_audio_position(state.clone()) {
            // emit position event
            event_emitter.emit_position(audio_position);
        }

        // update 25 times a second
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

    // test if the get_audio_position returns None if we are not playing
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
