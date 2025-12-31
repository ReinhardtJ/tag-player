// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod audio;
pub mod music_library;

use std::path::Path;
use crate::music_library::{gather_music_library, Library};
use audio::{AudioCommand, AudioPlayer};
use std::sync::mpsc::channel;
use std::thread;
use tauri::State;

#[tauri::command]
fn load_and_play(path: String, state: State<AudioPlayer>) -> Result<(), String> {
    state
        .sender
        .send(AudioCommand::LoadAndPlay(path))
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn toggle_playback(state: State<AudioPlayer>) -> Result<(), String> {
    state
        .sender
        .send(AudioCommand::TogglePlayback)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_music_library(path: String) -> Library {
    gather_music_library(Path::new(&path))
}

#[tauri::command]
fn volume_change(volume: f32, state: State<AudioPlayer>) -> Result<(), String> {
    state
        .sender
        .send(AudioCommand::VolumeChange(volume))
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (sender, receiver) = channel();
    thread::spawn(move || audio::audio_thread(receiver));

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AudioPlayer { sender })
        .invoke_handler(tauri::generate_handler![
            load_and_play,
            toggle_playback,
            get_music_library,
            volume_change
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
