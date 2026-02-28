use crate::tags::reading_tags;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

#[derive(serde::Serialize)]
pub struct Song {
    pub path: String,
    pub name: String,
    pub duration_millis: u32,
    pub tags: HashMap<String, String>,
    pub cover_base64: Option<String>,
}

#[derive(serde::Serialize)]
pub struct Library {
    pub songs: Vec<Song>,
    pub errors: Vec<String>,
}

pub fn read_music_library(library_dir: &Path) -> Library {
    let mut songs = Vec::new();
    let mut errors = Vec::new();

    for entry_result in WalkDir::new(library_dir).into_iter() {
        // Add WalkDir errors to errors
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(entry) => {
                errors.push(format!("Directory traversal error: {}", entry));
                continue;
            }
        };

        // skip non-music files
        if !entry.path().is_file() || !reading_tags::is_music_file(&entry) {
            continue;
        }

        let path = match entry.path().to_str() {
            Some(p) => p.to_string(),
            None => {
                errors.push(format!("Invalid UTF-8 in path: {:?}", entry.path()));
                continue;
            }
        };

        let name = match entry.file_name().to_str() {
            Some(n) => n.to_string(),
            None => continue,
        };

        match reading_tags::read_audio_file_properties(entry.path()) {
            Ok(properties) => songs.push(Song {
                path,
                name,
                duration_millis: properties.duration_millis,
                tags: properties.tags,
                cover_base64: properties.cover_base64,
            }),
            Err(e) => errors.push(format!("{}, {:?}", path, e)),
        }
    }

    Library { songs, errors }
}
