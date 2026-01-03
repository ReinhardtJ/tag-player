use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use crate::audio::shared::{get_duration_seconds, PlaybackState};

pub fn position_updater_thread(state: Arc<Mutex<PlaybackState>>, app_handle: AppHandle) {
    loop {
        // update 5 times a second
        thread::sleep(Duration::from_millis(200));

        let (position, duration, is_playing) = {
            let state = state.lock().unwrap();
            (
                get_position_seconds(state.current_position_samples, state.sample_rate),
                get_duration_seconds(state.duration_samples, state.sample_rate),
                state.is_playing,
            )
        };

        if !is_playing {
            // sleep longer when not playing to reduce CPU usage
            thread::sleep(Duration::from_secs(1));
            continue;
        }

        // emit position event
        let _ = app_handle.emit(
            "playback:position",
            AudioPosition {
                position_seconds: position,
                duration_seconds: duration.unwrap_or(0.0),
            },
        );
    }
}

fn get_position_seconds(current_position_samples: u64, sample_rate: u32) -> f64 {
    current_position_samples as f64 / sample_rate as f64
}

#[derive(serde::Serialize, Clone)]
pub struct AudioPosition {
    pub position_seconds: f64,
    pub duration_seconds: f64,
}