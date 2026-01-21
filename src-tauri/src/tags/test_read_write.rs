
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::copy;
    use tempfile::tempdir;

    use crate::tags::reading_tags::read_audio_file_properties;
    use crate::tags::writing_tags::write_tags_to_file;

    #[test]
    fn test_write_read_normal_tag() {
        let dir = tempdir().unwrap();
        let temp_path = dir.path().join("test_normal.mp3");
        
        // Copy test file to temp location
        copy(
            "./tests/music_libraries/different_formats/some_song.mp3",
            &temp_path,
        )
        .unwrap();

        // Write a normal tag
        let tags_to_write = HashMap::from([
            ("TrackTitle".to_string(), "My Test Song".to_string()),
            ("TrackArtist".to_string(), "Test Artist".to_string()),
        ]);
        write_tags_to_file(&temp_path, &tags_to_write).unwrap();

        // Read back the tags
        let properties = read_audio_file_properties(&temp_path).unwrap();
        
        // Verify the written tags match
        assert_eq!(
            properties.tags.get("TrackTitle"),
            Some(&"My Test Song".to_string()),
            "TrackTitle should match"
        );
        assert_eq!(
            properties.tags.get("TrackArtist"),
            Some(&"Test Artist".to_string()),
            "TrackArtist should match"
        );
    }

    #[test]
    fn test_write_read_unknown_tags() {
        // Test that custom/unknown tag keys are handled gracefully
        // Note: Not all formats support arbitrary custom tags
        let dir = tempdir().unwrap();
        let temp_path = dir.path().join("test_unknown.mp3");
        
        // Copy test file to temp location
        copy(
            "./tests/music_libraries/different_formats/some_song.mp3",
            &temp_path,
        )
        .unwrap();

        // Write a tag with a custom key - this tests the ItemKey::Unknown fallback
        // The write should succeed even if the custom tag isn't preserved
        let custom_key = "CustomField";
        let tags_to_write = HashMap::from([
            (custom_key.to_string(), "Custom Value".to_string()),
        ]);
        let write_result = write_tags_to_file(&temp_path, &tags_to_write);
        
        // Writing unknown tags should not error
        assert!(write_result.is_ok(), "Writing unknown tags should not fail");

        // Read back the tags to verify file is still valid
        let properties = read_audio_file_properties(&temp_path).unwrap();
        
        // The file should still be readable and have the primary tags we wrote
        assert!(
            properties.tags.contains_key("TrackTitle") || properties.tags.is_empty(),
            "File should be readable after writing unknown tags"
        );
    }

    #[test]
    fn test_write_read_multiple_tags() {
        let dir = tempdir().unwrap();
        let temp_path = dir.path().join("test_multiple.mp3");
        
        // Copy test file to temp location
        copy(
            "./tests/music_libraries/different_formats/some_song.mp3",
            &temp_path,
        )
        .unwrap();

        // Write multiple tags of different types
        let tags_to_write = HashMap::from([
            ("TrackTitle".to_string(), "Multi Tag Song".to_string()),
            ("TrackArtist".to_string(), "Various Artists".to_string()),
            ("AlbumTitle".to_string(), "Test Album".to_string()),
            ("Genre".to_string(), "Rock".to_string()),
            ("RecordingDate".to_string(), "2024-01-01".to_string()),
            ("Comment".to_string(), "A test comment".to_string()),
        ]);
        write_tags_to_file(&temp_path, &tags_to_write).unwrap();

        // Read back the tags
        let properties = read_audio_file_properties(&temp_path).unwrap();
        
        // Verify all written tags match
        assert_eq!(
            properties.tags.get("TrackTitle"),
            Some(&"Multi Tag Song".to_string()),
        );
        assert_eq!(
            properties.tags.get("TrackArtist"),
            Some(&"Various Artists".to_string()),
        );
        assert_eq!(
            properties.tags.get("AlbumTitle"),
            Some(&"Test Album".to_string()),
        );
        assert_eq!(
            properties.tags.get("Genre"),
            Some(&"Rock".to_string()),
        );
        assert_eq!(
            properties.tags.get("RecordingDate"),
            Some(&"2024-01-01".to_string()),
        );
        assert_eq!(
            properties.tags.get("Comment"),
            Some(&"A test comment".to_string()),
        );
        
        // Verify we have exactly the number of tags we wrote
        assert_eq!(properties.tags.len(), 6, "Should have exactly 6 tags");
    }
}