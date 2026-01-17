use anyhow::Context;
use lofty::config::{ParseOptions, ParsingMode};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::prelude::ItemKey;
use lofty::probe::Probe;
use std::collections::HashMap;
use std::path::Path;
use walkdir::DirEntry;

pub struct AudioFileProperties {
    pub tags: HashMap<String, String>,
    pub duration_millis: u32,
}

pub fn read_audio_file_properties(path: &Path) -> anyhow::Result<AudioFileProperties> {
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

    Ok(AudioFileProperties {
        tags,
        duration_millis,
    })
}

pub fn is_music_file(entry: &DirEntry) -> bool {
    if !entry.path().is_file() {
        return false;
    }

    let extension = get_file_extension(entry.path());
    extension
        .map(|ext| matches!(ext.as_str(), "mp3" | "flac" | "wav"))
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
}
