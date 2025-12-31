use anyhow::{Context, Result};
use lofty::prelude::*;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
#[derive(serde::Serialize, Default)]
pub struct Tags {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album_artist: Option<String>,
    pub album: Option<String>,
    pub date: Option<String>,
    pub genre: Option<String>,
    pub mood: Option<String>,
    pub track_number: Option<u32>,
}

#[derive(serde::Serialize)]
pub struct Song {
    pub path: String,
    pub name: String,
    pub tags: Tags,
}

#[derive(serde::Serialize)]
pub struct Library {
    pub songs: Vec<Song>,
    pub errors: Vec<String>,
}

pub fn gather_music_library(library_dir: &Path) -> Library {
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
        if !entry.path().is_file() || !is_music_file(&entry) {
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

        match read_tags(entry.path()) {
            Ok(tags) => songs.push(Song { path, name, tags }),
            Err(e) => errors.push(format!("{}, {:?}", path, e)),
        }
    }

    Library { songs, errors }
}

fn read_tags(path: &Path) -> Result<Tags> {
    let tagged_file = lofty::read_from_path(path)
        .with_context(|| format!("Failed to read audio file: {}", path.display()))?;

    let Some(tag) = tagged_file.primary_tag() else {
        return Ok(Tags::default())
    };

    Ok(Tags {
        title: tag.title().map(|title| title.to_string()),
        artist: tag.artist().map(|artist| artist.to_string()),
        album_artist: tag
            .get_string(&ItemKey::AlbumArtist)
            .map(|album_artist| album_artist.to_string()),
        album: tag.album().map(|album| album.to_string()),
        date: tag
            .get_string(&ItemKey::RecordingDate)
            .map(|date| date.to_string()),
        genre: tag.genre().map(|genre| genre.to_string()),
        mood: tag.get_string(&ItemKey::Mood).map(|mood| mood.to_string()),
        track_number: tag.track(),
    })
}

fn is_music_file(entry: &DirEntry) -> bool {
    if !entry.path().is_file() {
        return false;
    }

    entry
        .path()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| matches!(ext.to_lowercase().as_str(), "mp3" | "wav" | "flac"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_tags_nonexistent_file() {
        // Reading tags from a non-existent file should return appropriate IO error
        assert!(read_tags(Path::new("/nonexistent/file.mp3")).is_err());
    }
}

