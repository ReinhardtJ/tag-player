use anyhow::{Context, Result};
use lofty::config::{ParseOptions, ParsingMode, WriteOptions};
use lofty::prelude::*;
use lofty::probe::Probe;
use lofty::tag::Tag;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

#[derive(serde::Serialize, serde::Deserialize, Default)]
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
    pub duration_millis: u32,
    pub tags: Tags,
}

#[derive(serde::Serialize)]
pub struct Library {
    pub songs: Vec<Song>,
    pub errors: Vec<String>,
}

struct Properties {
    tags: Tags,
    duration_millis: u32,
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

        match read_audio_file_properties(entry.path()) {
            Ok(properties) => songs.push(Song {
                path,
                name,
                duration_millis: properties.duration_millis,
                tags: properties.tags,
            }),
            Err(e) => errors.push(format!("{}, {:?}", path, e)),
        }
    }

    Library { songs, errors }
}

fn read_audio_file_properties(path: &Path) -> Result<Properties> {
    let parse_options = ParseOptions::new().parsing_mode(ParsingMode::Relaxed);

    let tagged_file = Probe::open(path)?
        .options(parse_options)
        .read()
        .with_context(|| format!("Failed to read audio file: {}", path.display()))?;

    // Get duration from audio properties
    let duration_millis = tagged_file.properties().duration().as_millis() as u32;

    let Some(tag) = tagged_file.primary_tag() else {
        return Ok(Properties {
            tags: Tags::default(),
            duration_millis,
        });
    };

    let tags = Tags {
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
    };

    Ok(Properties {
        tags,
        duration_millis,
    })
}

fn is_music_file(entry: &DirEntry) -> bool {
    if !entry.path().is_file() {
        return false;
    }

    let extension = get_file_extension(entry.path());
    extension
        .map(|ext| matches!(ext.as_str(), "mp3" | "flac" | "wav"))
        .unwrap_or(false)
}

pub fn write_tags_to_file(path: &Path, tags: &Tags) -> Result<()> {
    let mut tagged_file = lofty::read_from_path(path)
        .with_context(|| format!("Failed to read audio file: {}", path.display()))?;

    // Get or create primary tag
    let tag = match tagged_file.primary_tag_mut() {
        Some(tag) => tag,
        None => {
            // If no tag exists, create one based on file type
            let file_type = tagged_file.file_type();
            let tag_type = file_type.primary_tag_type();
            tagged_file.insert_tag(Tag::new(tag_type));
            tagged_file.primary_tag_mut().unwrap()
        }
    };

    // Clear existing tags and set new ones
    tag.clear();

    if let Some(title) = &tags.title {
        tag.set_title(title.to_string());
    }
    if let Some(artist) = &tags.artist {
        tag.set_artist(artist.to_string());
    }
    if let Some(album_artist) = &tags.album_artist {
        tag.insert_text(ItemKey::AlbumArtist, album_artist.to_string());
    }
    if let Some(album) = &tags.album {
        tag.set_album(album.to_string());
    }
    if let Some(date) = &tags.date {
        tag.insert_text(ItemKey::RecordingDate, date.to_string());
    }
    if let Some(genre) = &tags.genre {
        tag.set_genre(genre.to_string());
    }
    if let Some(mood) = &tags.mood {
        tag.insert_text(ItemKey::Mood, mood.to_string());
    }
    if let Some(track_number) = tags.track_number {
        tag.set_track(track_number);
    }

    // Save changes to file
    tagged_file
        .save_to_path(path, WriteOptions::default())
        .with_context(|| format!("Failed to save tags to file: {}", path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_tags_nonexistent_file() {
        // Reading tags from a non-existent file should return appropriate IO error
        let result = read_audio_file_properties(Path::new("/nonexistent/file.mp3"));
        assert!(result.is_err());
    }
}

pub fn get_file_extension(path: &Path) -> Option<String> {
    path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
}