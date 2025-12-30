use walkdir::{DirEntry, WalkDir};

#[derive(serde::Serialize)]
pub struct MusicFile {
    pub path: String,
    pub name: String,
}

pub fn gather_music_library(path: String) -> Vec<MusicFile> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| is_music_file(entry))
        .map(|entry| MusicFile {
            path: entry.path().to_str().unwrap().to_string(),
            name: entry.file_name().to_str().unwrap().to_string(),
        })
        .collect()
}

fn is_music_file(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| matches!(ext.to_lowercase().as_str(), "mp3" | "wav" | "flac"))
        .unwrap_or(false)
}
