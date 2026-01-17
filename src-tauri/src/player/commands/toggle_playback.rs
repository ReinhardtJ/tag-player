use std::sync::{Arc, Mutex};
use crate::player::shared::PlaybackState;

pub fn toggle_playback(state: &Arc<Mutex<PlaybackState>>) {
    let mut state = state.lock().unwrap();
    state.is_paused = !state.is_paused;
    println!(
        "Playback {}",
        if state.is_paused { "paused" } else { "resumed" }
    );
}
