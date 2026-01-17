use std::sync::{Arc, Mutex};
use crate::player::shared::PlaybackState;

pub fn change_volume(state: &Arc<Mutex<PlaybackState>>, volume: f32) {
    let mut state = state.lock().unwrap();
    state.volume = volume;
    println!("Volume: {}", volume);
}
