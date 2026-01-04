use crate::audio::shared::PlaybackState;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub fn position_updater_thread(state: Arc<Mutex<PlaybackState>>, app_handle: AppHandle) {
    loop {
        let (position, is_playing) = {
            let state = state.lock().unwrap();
            (
                get_position_seconds(state.current_position_samples, state.sample_rate),
                state.is_playing,
            )
        };

        if !is_playing {
            thread::sleep(Duration::from_millis(40));
            continue;
        }

        // emit position event
        let _ = app_handle.emit(
            "playback:position",
            AudioPosition {
                position_seconds: position,
            },
        );

        // update 25 times a second
        thread::sleep(Duration::from_millis(40));
    }
}

fn get_position_seconds(current_position_samples: u64, sample_rate: u32) -> f64 {
    current_position_samples as f64 / sample_rate as f64
}

#[derive(serde::Serialize, Clone)]
pub struct AudioPosition {
    pub position_seconds: f64,
}
