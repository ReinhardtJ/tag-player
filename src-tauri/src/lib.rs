// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod music_library;
mod audio;

use crate::music_library::{gather_music_library, Library};
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use tauri::{Manager, State};
use crate::audio::audio_thread::audio_thread;
use crate::audio::shared::AudioCommand;

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

#[tauri::command]
fn seek(position_millis: u32, state: State<AudioPlayer>) -> Result<(), String> {
    println!("in seek command handler");
    // convert millis to fractional seconds
    let position_seconds = position_millis as f64 / 1000f64;
    state
        .sender
        .send(AudioCommand::Seek(position_seconds))
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let (sender, receiver) = channel();
            let app_handle = app.handle().clone();

            // spawn audio thread with app handle for event emission
            thread::spawn(move || audio_thread(receiver, app_handle));

            app.manage(AudioPlayer { sender });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_and_play,
            toggle_playback,
            get_music_library,
            volume_change,
            seek
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct AudioPlayer {
    pub sender: Sender<AudioCommand>,
}