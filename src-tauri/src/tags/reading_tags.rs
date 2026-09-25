use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use lofty::config::{ParseOptions, ParsingMode};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::picture::PictureType;
use lofty::prelude::ItemKey;
use lofty::probe::Probe;
use std::collections::HashMap;
use std::path::Path;
use lofty::tag::Tag;
use walkdir::DirEntry;

pub struct AudioFileProperties {
    pub tags: HashMap<String, String>,
    pub duration_millis: u32,
    pub cover_base64: Option<String>,
}

pub fn read_audio_file_properties(path: &Path) -> Result<AudioFileProperties> {
    let parse_options = ParseOptions::new().parsing_mode(ParsingMode::Relaxed);

    let tagged_file = Probe::open(path)?
        .options(parse_options)
        .read()
        .with_context(|| format!("Failed to read audio file: {}", path.display()))?;

    // Get duration from audio properties
    let duration_millis = tagged_file.properties().duration().as_millis() as u32;

    // return with empty tags if tag is None
    let Some(tag) = tagged_file.primary_tag() else {
        return Ok(AudioFileProperties {
            tags: HashMap::default(),
            duration_millis,
            cover_base64: None,
        });
    };

    let mut tags = HashMap::new();

    // read tags
    for item in tag.items() {
        if let Some(text) = item.value().text() {
            let key = match item.key() {
                ItemKey::Unknown(s) => s.clone(),
                other => format!("{:?}", other),
            };
            tags.insert(key, text.to_string());
        }
    }

    let cover_base64 = get_cover_as_base64(tag);

    Ok(AudioFileProperties {
        tags,
        duration_millis,
        cover_base64,
    })
}

/// Reads only the front cover (or first picture) of an audio file as a data URL.
/// Used to refresh the UI after a cover has been written.
pub fn extract_cover_data_url(path: &Path) -> Result<Option<String>> {
    let parse_options = ParseOptions::new().parsing_mode(ParsingMode::Relaxed);

    let tagged_file = Probe::open(path)?
        .options(parse_options)
        .read()
        .with_context(|| format!("Failed to read audio file: {}", path.display()))?;

    Ok(tagged_file.primary_tag().and_then(get_cover_as_base64))
}

fn get_cover_as_base64(tag: &Tag) -> Option<String> {
    tag
        .pictures()
        .iter()
        .find(|p| p.pic_type() == PictureType::CoverFront)
        .or_else(|| tag.pictures().first())
        .map(picture_to_data_url)
}

fn picture_to_data_url(picture: &lofty::picture::Picture) -> String {
    let mime = picture
        .mime_type()
        .map(|m| m.to_string())
        .unwrap_or_else(|| "image/jpeg".to_string());
    let data = STANDARD.encode(picture.data());
    format!("data:{};base64,{}", mime, data)
}

pub fn is_music_file(entry: &DirEntry) -> bool {
    if !entry.path().is_file() {
        return false;
    }

    let extension = get_file_extension(entry.path());
    extension
        .map(|ext| matches!(ext.as_str(), "mp3" | "flac" | "wav" | "m4a"))
        .unwrap_or(false)
}

pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tags::reading_tags::read_audio_file_properties;

    #[test]
    fn test_read_tags_nonexistent_file() {
        // Reading tags from a non-existent file should return appropriate IO error
        let result = read_audio_file_properties(Path::new("/nonexistent/file.mp3"));
        assert!(result.is_err());
    }

    #[test]
    fn test_read_tags_from_file_without_tags() {
        // Reading tags from a file without tags should return empty tags HashMap
        let result = read_audio_file_properties(Path::new(
            "./tests/music_libraries/different_formats/some_song.wav",
        ));
        assert!(result.is_ok());

        let properties = result.unwrap();
        assert!(
            properties.tags.is_empty(),
            "Expected empty tags for WAV file"
        );
        assert!(properties.duration_millis > 0, "Expected non-zero duration");
    }

    #[test]
    fn test_read_tags_from_m4a_file() {
        // Reading tags from an M4A file should succeed and report a duration.
        let result = read_audio_file_properties(Path::new(
            "./tests/music_libraries/different_formats/some_song.m4a",
        ));
        assert!(result.is_ok());

        let properties = result.unwrap();
        assert!(properties.duration_millis > 0, "Expected non-zero duration");
    }

    #[test]
    fn test_is_music_file_recognizes_m4a() {
        // The m4a extension must pass the music-file allowlist so it enters the library.
        let entry = walkdir::WalkDir::new("./tests/music_libraries/different_formats")
            .into_iter()
            .filter_map(|e| e.ok())
            .find(|e| e.file_name() == "some_song.m4a")
            .expect("m4a fixture not found");

        assert!(is_music_file(&entry));
    }

    #[test]
    fn test_extract_cover_data_url_without_cover() {
        // A file with no embedded picture should yield no data URL.
        let result = extract_cover_data_url(Path::new(
            "./tests/music_libraries/different_formats/some_song.wav",
        ));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_extract_cover_data_url_returns_written_cover() {
        use lofty::file::TaggedFileExt;
        use lofty::picture::MimeType;

        // Copy a fixture so the checked-in file is never modified.
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("some_song.mp3");
        std::fs::copy(
            "./tests/music_libraries/one_file_with_tags/some_song.mp3",
            &file_path,
        )
        .unwrap();

        // Embed the static cover fixture directly through lofty.
        let cover_bytes = std::fs::read("./tests/images/cover.png").unwrap();
        let mut tagged_file = lofty::read_from_path(&file_path).unwrap();
        tagged_file
            .primary_tag_mut()
            .unwrap()
            .push_picture(lofty::picture::Picture::new_unchecked(
                PictureType::CoverFront,
                Some(MimeType::Png),
                None,
                cover_bytes.clone(),
            ));
        tagged_file
            .save_to_path(&file_path, lofty::config::WriteOptions::default())
            .unwrap();

        // The extracted data URL should carry the exact embedded bytes.
        let data_url = extract_cover_data_url(&file_path).unwrap().unwrap();
        let prefix = "data:image/png;base64,";
        assert!(data_url.starts_with(prefix), "unexpected data URL: {data_url}");

        let decoded = STANDARD.decode(&data_url[prefix.len()..]).unwrap();
        assert_eq!(decoded, cover_bytes);
    }
}
