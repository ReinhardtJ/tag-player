use std::collections::HashMap;
use std::path::Path;
use anyhow::{Context, Result};
use lofty::config::WriteOptions;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::prelude::{ItemKey, TagExt};
use lofty::tag::{ItemValue, Tag, TagItem};

/// All supported tags defined once as (Display Name, ItemKey) tuples
const SUPPORTED_TAGS: [(&str, ItemKey); 103] = [
    // Standard keys
    ("AlbumTitle", ItemKey::AlbumTitle),
    ("SetSubtitle", ItemKey::SetSubtitle),
    ("ShowName", ItemKey::ShowName),
    ("ContentGroup", ItemKey::ContentGroup),
    ("TrackTitle", ItemKey::TrackTitle),
    ("TrackSubtitle", ItemKey::TrackSubtitle),
    ("OriginalAlbumTitle", ItemKey::OriginalAlbumTitle),
    ("OriginalArtist", ItemKey::OriginalArtist),
    ("OriginalLyricist", ItemKey::OriginalLyricist),
    ("AlbumTitleSortOrder", ItemKey::AlbumTitleSortOrder),
    ("AlbumArtistSortOrder", ItemKey::AlbumArtistSortOrder),
    ("TrackTitleSortOrder", ItemKey::TrackTitleSortOrder),
    ("TrackArtistSortOrder", ItemKey::TrackArtistSortOrder),
    ("ShowNameSortOrder", ItemKey::ShowNameSortOrder),
    ("ComposerSortOrder", ItemKey::ComposerSortOrder),
    ("AlbumArtist", ItemKey::AlbumArtist),
    ("TrackArtist", ItemKey::TrackArtist),
    ("TrackArtists", ItemKey::TrackArtists),
    ("Arranger", ItemKey::Arranger),
    ("Writer", ItemKey::Writer),
    ("Composer", ItemKey::Composer),
    ("Conductor", ItemKey::Conductor),
    ("Director", ItemKey::Director),
    ("Engineer", ItemKey::Engineer),
    ("Lyricist", ItemKey::Lyricist),
    ("MixDj", ItemKey::MixDj),
    ("MixEngineer", ItemKey::MixEngineer),
    ("MusicianCredits", ItemKey::MusicianCredits),
    ("Performer", ItemKey::Performer),
    ("Producer", ItemKey::Producer),
    ("Publisher", ItemKey::Publisher),
    ("Label", ItemKey::Label),
    ("InternetRadioStationName", ItemKey::InternetRadioStationName),
    ("InternetRadioStationOwner", ItemKey::InternetRadioStationOwner),
    ("Remixer", ItemKey::Remixer),
    // Track/Disc info
    ("DiscNumber", ItemKey::DiscNumber),
    ("DiscTotal", ItemKey::DiscTotal),
    ("TrackNumber", ItemKey::TrackNumber),
    ("TrackTotal", ItemKey::TrackTotal),
    ("Popularimeter", ItemKey::Popularimeter),
    ("ParentalAdvisory", ItemKey::ParentalAdvisory),
    // Date/Time
    ("RecordingDate", ItemKey::RecordingDate),
    ("Year", ItemKey::Year),
    ("ReleaseDate", ItemKey::ReleaseDate),
    ("OriginalReleaseDate", ItemKey::OriginalReleaseDate),
    // Identifiers
    ("Isrc", ItemKey::Isrc),
    ("Barcode", ItemKey::Barcode),
    ("CatalogNumber", ItemKey::CatalogNumber),
    ("Work", ItemKey::Work),
    ("Movement", ItemKey::Movement),
    ("MovementNumber", ItemKey::MovementNumber),
    ("MovementTotal", ItemKey::MovementTotal),
    ("MusicBrainzRecordingId", ItemKey::MusicBrainzRecordingId),
    ("MusicBrainzTrackId", ItemKey::MusicBrainzTrackId),
    ("MusicBrainzReleaseId", ItemKey::MusicBrainzReleaseId),
    ("MusicBrainzReleaseGroupId", ItemKey::MusicBrainzReleaseGroupId),
    ("MusicBrainzArtistId", ItemKey::MusicBrainzArtistId),
    ("MusicBrainzReleaseArtistId", ItemKey::MusicBrainzReleaseArtistId),
    ("MusicBrainzWorkId", ItemKey::MusicBrainzWorkId),
    // Flags
    ("FlagCompilation", ItemKey::FlagCompilation),
    ("FlagPodcast", ItemKey::FlagPodcast),
    // Technical
    ("FileType", ItemKey::FileType),
    ("FileOwner", ItemKey::FileOwner),
    ("TaggingTime", ItemKey::TaggingTime),
    ("Length", ItemKey::Length),
    ("OriginalFileName", ItemKey::OriginalFileName),
    ("OriginalMediaType", ItemKey::OriginalMediaType),
    // Encoding
    ("EncodedBy", ItemKey::EncodedBy),
    ("EncoderSoftware", ItemKey::EncoderSoftware),
    ("EncoderSettings", ItemKey::EncoderSettings),
    ("EncodingTime", ItemKey::EncodingTime),
    // ReplayGain
    ("ReplayGainAlbumGain", ItemKey::ReplayGainAlbumGain),
    ("ReplayGainAlbumPeak", ItemKey::ReplayGainAlbumPeak),
    ("ReplayGainTrackGain", ItemKey::ReplayGainTrackGain),
    ("ReplayGainTrackPeak", ItemKey::ReplayGainTrackPeak),
    // URLs
    ("AudioFileUrl", ItemKey::AudioFileUrl),
    ("AudioSourceUrl", ItemKey::AudioSourceUrl),
    ("CommercialInformationUrl", ItemKey::CommercialInformationUrl),
    ("CopyrightUrl", ItemKey::CopyrightUrl),
    ("TrackArtistUrl", ItemKey::TrackArtistUrl),
    ("RadioStationUrl", ItemKey::RadioStationUrl),
    ("PaymentUrl", ItemKey::PaymentUrl),
    ("PublisherUrl", ItemKey::PublisherUrl),
    // Other
    ("Genre", ItemKey::Genre),
    ("InitialKey", ItemKey::InitialKey),
    ("Color", ItemKey::Color),
    ("Mood", ItemKey::Mood),
    ("Bpm", ItemKey::Bpm),
    ("IntegerBpm", ItemKey::IntegerBpm),
    ("CopyrightMessage", ItemKey::CopyrightMessage),
    ("License", ItemKey::License),
    // Podcast-specific
    ("PodcastDescription", ItemKey::PodcastDescription),
    ("PodcastSeriesCategory", ItemKey::PodcastSeriesCategory),
    ("PodcastUrl", ItemKey::PodcastUrl),
    ("PodcastGlobalUniqueId", ItemKey::PodcastGlobalUniqueId),
    ("PodcastKeywords", ItemKey::PodcastKeywords),
    // Comments/Descriptions
    ("Comment", ItemKey::Comment),
    ("Description", ItemKey::Description),
    ("Language", ItemKey::Language),
    ("Script", ItemKey::Script),
    ("Lyrics", ItemKey::Lyrics),
    // Apple-specific
    ("AppleXid", ItemKey::AppleXid),
    ("AppleId3v2ContentGroup", ItemKey::AppleId3v2ContentGroup),
];

pub fn write_tags_to_file(path: &Path, tags: &HashMap<String, String>) -> Result<()> {
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

    // Clear existing tags
    tag.clear();

    // Set new tags
    for (tag_key, tag_value) in tags {
        let item_key = parse_item_key(tag_key);
        let tag_item = TagItem::new(item_key, ItemValue::Text(tag_value.clone()));
        tag.insert(tag_item);
    }

    // Save changes to file
    tagged_file
        .save_to_path(path, WriteOptions::default())
        .with_context(|| format!("Failed to save tags to file: {}", path.display()))?;

    Ok(())
}

/// Converts a string to an ItemKey (case-insensitive).
/// Returns ItemKey::Unknown(s) if the string doesn't match any known ItemKey variant.
pub fn parse_item_key(s: &str) -> ItemKey {
    let s_lower = s.to_lowercase();
    for (name, item_key) in SUPPORTED_TAGS {
        if name.to_lowercase() == s_lower {
            return item_key.clone();
        }
    }
    ItemKey::Unknown(s.to_string())
}

/// Get all supported tag names as a vector of strings.
/// Returns the list of all ItemKey names that can be used for writing tags.
pub fn get_supported_tags() -> Vec<String> {
    SUPPORTED_TAGS.iter().map(|(name, _)| name.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_tag_to_nonexistent_file() {
        // Writing tags to a non-existent file should return an error
        let tags = HashMap::from([
            ("TrackTitle".to_string(), "Test Title".to_string()),
        ]);
        let result = write_tags_to_file(Path::new("/nonexistent/path/file.mp3"), &tags);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_item_key_case_insensitive() {
        // Test that parse_item_key handles various case formats
        assert!(matches!(parse_item_key("AlbumArtist"), ItemKey::AlbumArtist));
        assert!(matches!(parse_item_key("albumartist"), ItemKey::AlbumArtist));
        assert!(matches!(parse_item_key("ALBUMARTIST"), ItemKey::AlbumArtist));
        assert!(matches!(parse_item_key("aLbUmArTiSt"), ItemKey::AlbumArtist));

        assert!(matches!(parse_item_key("TrackTitle"), ItemKey::TrackTitle));
        assert!(matches!(parse_item_key("tracktitle"), ItemKey::TrackTitle));
        assert!(matches!(parse_item_key("TRACKTITLE"), ItemKey::TrackTitle));

        assert!(matches!(parse_item_key("Genre"), ItemKey::Genre));
        assert!(matches!(parse_item_key("genre"), ItemKey::Genre));
        assert!(matches!(parse_item_key("GENRE"), ItemKey::Genre));

        // Test unknown key
        match parse_item_key("UnknownTag") {
            ItemKey::Unknown(s) => assert_eq!(s, "UnknownTag"),
            _ => panic!("Expected Unknown variant"),
        }
    }
}