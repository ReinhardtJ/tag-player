// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod player;
pub mod read_music_library;
mod tags;
mod decoder;
mod audio;
pub mod musicbrainz;
mod musicbrainz_tag_mapping;

use crate::musicbrainz::search_song_on_musicbrainz;
use crate::musicbrainz_tag_mapping::recording_to_tags;
use crate::player::shared::AudioPlayerCommand;
use crate::player::threads::player_thread::player_thread;
use crate::read_music_library::{read_music_library, Library, Song};
use crate::tags::writing_tags::{write_tags_to_file, get_supported_tags as get_supported_tags_list};
use crate::tags::covers::{write_cover_to_file};
use crate::tags::reading_tags::extract_cover_data_url;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{mpsc, Arc};
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
fn write_tags(path: String, tags: HashMap<String, String>) -> Result<(), String> {
    write_tags_to_file(Path::new(&path), &tags).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn write_cover(path: String, cover_path: Option<String>) -> Result<(), String> {
    write_cover_to_file(
        Path::new(&path),
        cover_path.as_deref().map(Path::new)
    ).map_err(|e| e.to_string())
}

/// Re-reads the embedded cover of an audio file so the frontend can refresh
/// its preview and list thumbnail after a cover write.
#[tauri::command]
fn read_cover_data_url(path: String) -> Result<Option<String>, String> {
    extract_cover_data_url(Path::new(&path)).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_supported_tags() -> Vec<String> {
    get_supported_tags_list()
}

#[tauri::command]
async fn search_musicbrainz(song: Song) -> Result<Vec<crate::musicbrainz::Recording>, String> {
    search_song_on_musicbrainz(&song).await
}

#[tauri::command]
fn musicbrainz_recording_to_tags(
    recording: crate::musicbrainz::Recording,
) -> HashMap<String, String> {
    recording_to_tags(&recording)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let (sender, receiver) = mpsc::channel();
            let app_handle_arc = Arc::new(app.handle().clone());

            // spawn player thread with app handle for event emission
            thread::spawn(move || player_thread(receiver, app_handle_arc));

            app.manage(AudioPlayer { sender });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_and_play,
            toggle_playback,
            get_music_library,
            volume_change,
            seek,
            write_tags,
            write_cover,
            read_cover_data_url,
            get_supported_tags,
            search_musicbrainz,
            musicbrainz_recording_to_tags
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct AudioPlayer {
    pub sender: mpsc::Sender<AudioPlayerCommand>,
}
