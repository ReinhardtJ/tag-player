// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod audio;
pub mod music_library;

use crate::audio::player::player_thread;
use crate::audio::shared::AudioPlayerCommand;
use crate::music_library::{read_music_library, write_tags_to_file, Library, Tags};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use tauri::{Manager, State};

#[tauri::command]
fn load_and_play(path: String, audio_player: State<AudioPlayer>) -> Result<(), String> {
    audio_player
        .sender
        .send(AudioPlayerCommand::LoadAndPlay(path))
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn toggle_playback(audio_player: State<AudioPlayer>) -> Result<(), String> {
    audio_player
        .sender
        .send(AudioPlayerCommand::TogglePlayback)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_music_library(path: String) -> Library {
    read_music_library(Path::new(&path))
}

#[tauri::command]
fn volume_change(volume: f32, audio_player: State<AudioPlayer>) -> Result<(), String> {
    audio_player
        .sender
        .send(AudioPlayerCommand::VolumeChange(volume))
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
        .send(AudioPlayerCommand::Seek(position_seconds))
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn write_tags(path: String, tags: Tags) -> Result<(), String> {
    write_tags_to_file(Path::new(&path), &tags).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let (sender, receiver) = mpsc::channel();
            let app_handle = app.handle().clone();

            // spawn audio thread with app handle for event emission
            thread::spawn(move || player_thread(receiver, app_handle));

            app.manage(AudioPlayer { sender });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_and_play,
            toggle_playback,
            get_music_library,
            volume_change,
            seek,
            write_tags
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct AudioPlayer {
    pub sender: mpsc::Sender<AudioPlayerCommand>,
}
