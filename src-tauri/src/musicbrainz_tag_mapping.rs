use crate::musicbrainz::{
    Artist, ArtistCredit, Genre, Label, Recording, Release, ReleaseEvent, ReleaseGroup,
    ReleaseMedia, Track,
};
use std::collections::HashMap;

/// Maps a MusicBrainz recording to a HashMap of tags in lofty format.
///
/// MusicBrainz tags mapped to lofty equivalents:
/// - album → AlbumTitle
/// - albumartist → AlbumArtist
/// - albumartistsort → AlbumArtistSortOrder
/// - artist → TrackArtist
/// - artists → TrackArtists
/// - artistsort → TrackArtistSortOrder
/// - asin → Asin
/// - barcode → Barcode
/// - catalognumber → CatalogNumber
/// - comment → Comment (from release disambiguation)
/// - compilation → FlagCompilation
/// - date → ReleaseDate
/// - discnumber → DiscNumber
/// - discsubtitle → SetSubtitle
/// - isrc → Isrc
/// - label → Label
/// - media → Media
/// - musicbrainz_albumartistid → MusicBrainzReleaseArtistId
/// - musicbrainz_albumid → MusicBrainzReleaseId
/// - musicbrainz_artistid → MusicBrainzArtistId
/// - musicbrainz_recordingid → MusicBrainzRecordingId
/// - musicbrainz_releasegroupid → MusicBrainzReleaseGroupId
/// - musicbrainz_trackid → MusicBrainzTrackId
/// - originaldate → OriginalReleaseDate
/// - originalyear → (derived from OriginalReleaseDate)
/// - releasecountry → ReleaseCountry
/// - releasestatus → ReleaseStatus
/// - releasetype → ReleaseType
/// - script → Script
/// - title → TrackTitle
/// - totaldiscs → DiscTotal
/// - totaltracks → TrackTotal
/// - tracknumber → TrackNumber
pub fn recording_to_tags(recording: &Recording) -> HashMap<String, String> {
    let mut tags = HashMap::new();

    // Basic recording info
    tags.insert("TrackTitle".to_string(), recording.title.clone());
    tags.insert("MusicBrainzRecordingId".to_string(), recording.id.clone());

    // ISRCs
    if let Some(isrcs) = &recording.isrcs {
        if let Some(first_isrc) = isrcs.first() {
            tags.insert("Isrc".to_string(), first_isrc.clone());
        }
    }

    // Artist info from recording artist_credit
    if let Some(artist_credit) = &recording.artist_credit {
        let artist_names: Vec<String> = artist_credit.iter().map(|ac| ac.name.clone()).collect();
        tags.insert("TrackArtist".to_string(), artist_names.join(", "));

        // TrackArtists (multi-value)
        let track_artists: Vec<String> = artist_credit.iter().map(|ac| ac.name.clone()).collect();
        tags.insert("TrackArtists".to_string(), track_artists.join("; "));

        // Artist sort names
        let artist_sort_names: Vec<String> = artist_credit
            .iter()
            .map(|ac| {
                ac.artist
                    .sort_name
                    .clone()
                    .unwrap_or_else(|| ac.name.clone())
            })
            .collect();
        tags.insert(
            "TrackArtistSortOrder".to_string(),
            artist_sort_names.join(", "),
        );

        // MusicBrainz Artist IDs (multi-value)
        let artist_ids: Vec<String> = artist_credit
            .iter()
            .map(|ac| ac.artist.id.clone())
            .collect();
        tags.insert("MusicBrainzArtistId".to_string(), artist_ids.join("; "));
    } else if let Some(releases) = &recording.releases {
        // Fallback to release artist if no recording artist
        if let Some(first_release) = releases.first() {
            if let Some(artist_credit) = &first_release.artist_credit {
                let artist_names: Vec<String> =
                    artist_credit.iter().map(|ac| ac.name.clone()).collect();
                tags.insert("TrackArtist".to_string(), artist_names.join(", "));
                tags.insert("TrackArtists".to_string(), artist_names.join("; "));

                let artist_ids: Vec<String> = artist_credit
                    .iter()
                    .map(|ac| ac.artist.id.clone())
                    .collect();
                tags.insert("MusicBrainzArtistId".to_string(), artist_ids.join("; "));
            }
        }
    }

    // Release info
    if let Some(releases) = &recording.releases {
        if let Some(first_release) = releases.first() {
            // Album title
            tags.insert("AlbumTitle".to_string(), first_release.title.clone());
            tags.insert("MusicBrainzReleaseId".to_string(), first_release.id.clone());

            // ASIN
            if let Some(asin) = &first_release.asin {
                tags.insert("Asin".to_string(), asin.clone());
            }

            // Barcode
            if let Some(barcode) = &first_release.barcode {
                if !barcode.is_empty() {
                    tags.insert("Barcode".to_string(), barcode.clone());
                }
            }

            // Release date
            if let Some(date) = &first_release.date {
                tags.insert("ReleaseDate".to_string(), date.clone());
                if let Some(year) = date.get(0..4) {
                    tags.insert("Year".to_string(), year.to_string());
                }
            }

            // Release group info
            if let Some(release_group) = &first_release.release_group {
                tags.insert(
                    "MusicBrainzReleaseGroupId".to_string(),
                    release_group.id.clone(),
                );

                // Original date from release group first release
                if let Some(first_release_date) = &release_group.first_release_date {
                    if !first_release_date.is_empty() {
                        tags.insert(
                            "OriginalReleaseDate".to_string(),
                            first_release_date.clone(),
                        );
                        if let Some(year) = first_release_date.get(0..4) {
                            tags.insert("OriginalYear".to_string(), year.to_string());
                        }
                    }
                }
            }

            // Labels and catalog numbers
            if let Some(labels) = &first_release.labels {
                let label_names: Vec<String> =
                    labels.iter().filter_map(|l| l.name.clone()).collect();
                if !label_names.is_empty() {
                    tags.insert("Label".to_string(), label_names.join("; "));
                }

                let catalog_numbers: Vec<String> = labels
                    .iter()
                    .filter_map(|l| l.catalog_number.clone())
                    .collect();
                if !catalog_numbers.is_empty() {
                    tags.insert("CatalogNumber".to_string(), catalog_numbers.join("; "));
                }
            }

            // Release events (country)
            if let Some(events) = &first_release.events {
                if let Some(first_event) = events.first() {
                    if let Some(country) = &first_event.country {
                        if !country.is_empty() {
                            tags.insert("ReleaseCountry".to_string(), country.clone());
                        }
                    }
                }
            }

            // Release status
            if let Some(status) = &first_release.status {
                tags.insert("ReleaseStatus".to_string(), status.clone());
            }

            // Release type (multi-value)
            if let Some(release_types) = &first_release.release_type {
                tags.insert("ReleaseType".to_string(), release_types.join("; "));
            }

            // Script
            if let Some(script) = &first_release.script {
                tags.insert("Script".to_string(), script.clone());
            }

            // Comment (from disambiguation)
            if let Some(comment) = &first_release.disambiguation {
                if !comment.is_empty() {
                    tags.insert("Comment".to_string(), comment.clone());
                }
            }

            // Release artist (album artist)
            if let Some(artist_credit) = &first_release.artist_credit {
                let album_artist_names: Vec<String> =
                    artist_credit.iter().map(|ac| ac.name.clone()).collect();
                tags.insert("AlbumArtist".to_string(), album_artist_names.join(", "));
                tags.insert("AlbumArtists".to_string(), album_artist_names.join("; "));

                let album_artist_ids: Vec<String> = artist_credit
                    .iter()
                    .map(|ac| ac.artist.id.clone())
                    .collect();
                tags.insert(
                    "MusicBrainzReleaseArtistId".to_string(),
                    album_artist_ids.join("; "),
                );

                // Album artist sort names
                let album_artist_sort: Vec<String> = artist_credit
                    .iter()
                    .map(|ac| {
                        ac.artist
                            .sort_name
                            .clone()
                            .unwrap_or_else(|| ac.name.clone())
                    })
                    .collect();
                tags.insert(
                    "AlbumArtistSortOrder".to_string(),
                    album_artist_sort.join(", "),
                );
            }

            // Media/track info
            if let Some(media) = &first_release.media {
                // Total tracks on this disc
                if let Some(first_media) = media.first() {
                    // Disc subtitle (from media format if available)
                    if let Some(format) = &first_media.format {
                        // Some formats include disc subtitle info
                        tags.insert("Media".to_string(), format.clone());
                    }

                    // Track info for first media
                    if let Some(tracks) = &first_media.tracks {
                        if let Some(first_track) = tracks.first() {
                            // Track number
                            if let Some(number) = &first_track.number {
                                tags.insert("TrackNumber".to_string(), number.clone());
                            }

                            // Disc number (position)
                            if let Some(position) = first_track.position {
                                tags.insert("DiscNumber".to_string(), position.to_string());
                            }

                            // MusicBrainz Track ID
                            if let Some(track_id) = &first_track.id {
                                tags.insert("MusicBrainzTrackId".to_string(), track_id.clone());
                            }

                            // Track ISRC (from track)
                            if let Some(track_isrcs) = &first_track.isrcs {
                                if let Some(first_isrc) = track_isrcs.first() {
                                    // Only override if we don't already have one from recording
                                    if !tags.contains_key("Isrc") {
                                        tags.insert("Isrc".to_string(), first_isrc.clone());
                                    }
                                }
                            }
                        }

                        // Total tracks on this disc
                        let track_count = tracks.len();
                        if track_count > 0 {
                            tags.insert("TrackTotal".to_string(), track_count.to_string());
                        }
                    }

                    // Track count from media (may differ from actual tracks)
                    if let Some(track_count) = first_media.track_count {
                        // Only set if not already set from tracks
                        if !tags.contains_key("TrackTotal") {
                            tags.insert("TrackTotal".to_string(), track_count.to_string());
                        }
                    }
                }

                // Total discs
                let total_discs = media.len();
                if total_discs > 0 {
                    tags.insert("DiscTotal".to_string(), total_discs.to_string());
                }
            }
        }
    }

    // Genres (from recording)
    if let Some(genres) = &recording.genres {
        if let Some(first_genre) = genres.first() {
            tags.insert("Genre".to_string(), first_genre.name.clone());
        }
    }

    // Tags (from recording) - these are user tags, not genre tags
    // We could use these for genre if no genre is set
    if let Some(mb_tags) = &recording.tags {
        if !mb_tags.is_empty() && !tags.contains_key("Genre") {
            if let Some(first_tag) = mb_tags.first() {
                tags.insert("Genre".to_string(), first_tag.name.clone());
            }
        }
    }

    // Compilation flag - set if there are multiple track artists
    if let Some(artist_credit) = &recording.artist_credit {
        let has_multiple_artists =
            artist_credit.len() > 1 || artist_credit.iter().any(|ac| ac.joinphrase.is_some());
        if has_multiple_artists {
            tags.insert("FlagCompilation".to_string(), "1".to_string());
        }
    }

    tags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recording_to_tags_basic() {
        let recording = Recording {
            id: "test-id".to_string(),
            title: "Test Song".to_string(),
            artist_credit: Some(vec![ArtistCredit {
                name: "Test Artist".to_string(),
                joinphrase: None,
                artist: Artist {
                    id: "artist-id".to_string(),
                    name: "Test Artist".to_string(),
                    sort_name: Some("Artist, Test".to_string()),
                    extra: HashMap::new(),
                },
                extra: HashMap::new(),
            }]),
            releases: Some(vec![Release {
                id: "release-id".to_string(),
                title: "Test Album".to_string(),
                date: Some("1991-09-24".to_string()),
                country: Some("US".to_string()),
                media: Some(vec![ReleaseMedia {
                    track_count: Some(12),
                    position: Some(1),
                    tracks: Some(vec![Track {
                        id: Some("track-id".to_string()),
                        number: Some("1".to_string()),
                        position: Some(1),
                        length: Some(180000),
                        isrcs: Some(vec!["USXXX1234567".to_string()]),
                        title: None,
                        extra: HashMap::new(),
                    }]),
                    format: Some("CD".to_string()),
                    format_id: None,
                    extra: HashMap::new(),
                }]),
                artist_credit: None,
                release_group: Some(ReleaseGroup {
                    id: "release-group-id".to_string(),
                    first_release_date: Some("1991-09-24".to_string()),
                    extra: HashMap::new(),
                }),
                events: Some(vec![ReleaseEvent {
                    date: Some("1991-09-24".to_string()),
                    country: Some("US".to_string()),
                    extra: HashMap::new(),
                }]),
                labels: Some(vec![Label {
                    name: Some("DGC Records".to_string()),
                    catalog_number: Some("DGCD-24425".to_string()),
                    label_code: Some(1234),
                    extra: HashMap::new(),
                }]),
                asin: Some("B0000001".to_string()),
                barcode: Some("0720642442525".to_string()),
                status: Some("Official".to_string()),
                release_type: Some(vec!["album".to_string(), "studio".to_string()]),
                script: Some("Latn".to_string()),
                disambiguation: Some("Remastered edition".to_string()),
                extra: HashMap::new(),
            }]),
            isrcs: Some(vec!["USXXX1234567".to_string()]),
            tags: None,
            genres: Some(vec![Genre {
                name: "Rock".to_string(),
                count: 10,
                extra: HashMap::new(),
            }]),
            disambiguation: None,
            first_release_date: None,
            extra: HashMap::new(),
        };

        let tags = recording_to_tags(&recording);

        // Basic info
        assert_eq!(tags.get("TrackTitle"), Some(&"Test Song".to_string()));
        assert_eq!(tags.get("TrackArtist"), Some(&"Test Artist".to_string()));
        assert_eq!(tags.get("AlbumTitle"), Some(&"Test Album".to_string()));

        // Dates
        assert_eq!(tags.get("Year"), Some(&"1991".to_string()));
        assert_eq!(tags.get("ReleaseDate"), Some(&"1991-09-24".to_string()));
        assert_eq!(
            tags.get("OriginalReleaseDate"),
            Some(&"1991-09-24".to_string())
        );

        // Track info
        assert_eq!(tags.get("TrackNumber"), Some(&"1".to_string()));
        assert_eq!(tags.get("DiscNumber"), Some(&"1".to_string()));
        assert_eq!(tags.get("TrackTotal"), Some(&"1".to_string()));
        assert_eq!(tags.get("DiscTotal"), Some(&"1".to_string()));

        // MusicBrainz IDs
        assert_eq!(
            tags.get("MusicBrainzRecordingId"),
            Some(&"test-id".to_string())
        );
        assert_eq!(
            tags.get("MusicBrainzArtistId"),
            Some(&"artist-id".to_string())
        );
        assert_eq!(
            tags.get("MusicBrainzReleaseId"),
            Some(&"release-id".to_string())
        );
        assert_eq!(
            tags.get("MusicBrainzReleaseGroupId"),
            Some(&"release-group-id".to_string())
        );
        assert_eq!(
            tags.get("MusicBrainzTrackId"),
            Some(&"track-id".to_string())
        );

        // Other tags
        assert_eq!(tags.get("Isrc"), Some(&"USXXX1234567".to_string()));
        assert_eq!(tags.get("Barcode"), Some(&"0720642442525".to_string()));
        assert_eq!(tags.get("Label"), Some(&"DGC Records".to_string()));
        assert_eq!(tags.get("CatalogNumber"), Some(&"DGCD-24425".to_string()));
        assert_eq!(tags.get("Script"), Some(&"Latn".to_string()));
        assert_eq!(tags.get("Asin"), Some(&"B0000001".to_string()));
        assert_eq!(tags.get("Comment"), Some(&"Remastered edition".to_string()));
        assert_eq!(tags.get("Genre"), Some(&"Rock".to_string()));
        assert_eq!(tags.get("ReleaseStatus"), Some(&"Official".to_string()));
        assert_eq!(tags.get("ReleaseType"), Some(&"album; studio".to_string()));
        assert_eq!(tags.get("ReleaseCountry"), Some(&"US".to_string()));

        // Sort orders
        assert_eq!(
            tags.get("TrackArtistSortOrder"),
            Some(&"Artist, Test".to_string())
        );
    }

    #[test]
    fn test_recording_to_tags_with_multiple_artists() {
        let recording = Recording {
            id: "recording-id".to_string(),
            title: "Song Title".to_string(),
            artist_credit: Some(vec![
                ArtistCredit {
                    name: "Primary Artist".to_string(),
                    joinphrase: Some(" feat. ".to_string()),
                    artist: Artist {
                        id: "artist-1".to_string(),
                        name: "Primary Artist".to_string(),
                        sort_name: Some("Artist, Primary".to_string()),
                        extra: HashMap::new(),
                    },
                    extra: HashMap::new(),
                },
                ArtistCredit {
                    name: "Featured Artist".to_string(),
                    joinphrase: None,
                    artist: Artist {
                        id: "artist-2".to_string(),
                        name: "Featured Artist".to_string(),
                        sort_name: Some("Artist, Featured".to_string()),
                        extra: HashMap::new(),
                    },
                    extra: HashMap::new(),
                },
            ]),
            releases: Some(vec![Release {
                id: "release-id".to_string(),
                title: "Album Title".to_string(),
                date: Some("2020-01-01".to_string()),
                country: None,
                media: Some(vec![ReleaseMedia {
                    track_count: Some(10),
                    position: Some(1),
                    tracks: Some(vec![Track {
                        id: None,
                        number: Some("1".to_string()),
                        position: Some(1),
                        length: Some(200000),
                        isrcs: None,
                        title: None,
                        extra: HashMap::new(),
                    }]),
                    format: Some("Digital".to_string()),
                    format_id: None,
                    extra: HashMap::new(),
                }]),
                artist_credit: Some(vec![ArtistCredit {
                    name: "Album Artist".to_string(),
                    joinphrase: None,
                    artist: Artist {
                        id: "album-artist-id".to_string(),
                        name: "Album Artist".to_string(),
                        sort_name: Some("Artist, Album".to_string()),
                        extra: HashMap::new(),
                    },
                    extra: HashMap::new(),
                }]),
                release_group: None,
                events: None,
                labels: None,
                asin: None,
                barcode: None,
                status: None,
                release_type: None,
                script: None,
                disambiguation: None,
                extra: HashMap::new(),
            }]),
            isrcs: None,
            tags: None,
            genres: None,
            disambiguation: None,
            first_release_date: None,
            extra: HashMap::new(),
        };

        let tags = recording_to_tags(&recording);

        // Multiple artists should be joined
        assert_eq!(
            tags.get("TrackArtist"),
            Some(&"Primary Artist, Featured Artist".to_string())
        );

        // Should be marked as compilation due to multiple artists
        assert_eq!(tags.get("FlagCompilation"), Some(&"1".to_string()));

        // Album artist should be separate
        assert_eq!(tags.get("AlbumArtist"), Some(&"Album Artist".to_string()));
    }

    #[test]
    fn test_recording_to_tags_multi_disc() {
        let recording = Recording {
            id: "recording-id".to_string(),
            title: "Song Title".to_string(),
            artist_credit: Some(vec![ArtistCredit {
                name: "Test Artist".to_string(),
                joinphrase: None,
                artist: Artist {
                    id: "artist-id".to_string(),
                    name: "Test Artist".to_string(),
                    sort_name: None,
                    extra: HashMap::new(),
                },
                extra: HashMap::new(),
            }]),
            releases: Some(vec![Release {
                id: "release-id".to_string(),
                title: "Album Title".to_string(),
                date: Some("2020-01-01".to_string()),
                country: None,
                media: Some(vec![
                    ReleaseMedia {
                        track_count: Some(10),
                        position: Some(1),
                        tracks: Some(vec![Track {
                            id: None,
                            number: Some("1".to_string()),
                            position: Some(1),
                            length: Some(180000),
                            isrcs: None,
                            title: None,
                            extra: HashMap::new(),
                        }]),
                        format: Some("CD".to_string()),
                        format_id: None,
                        extra: HashMap::new(),
                    },
                    ReleaseMedia {
                        track_count: Some(8),
                        position: Some(2),
                        tracks: Some(vec![Track {
                            id: None,
                            number: Some("1".to_string()),
                            position: Some(1),
                            length: Some(200000),
                            isrcs: None,
                            title: None,
                            extra: HashMap::new(),
                        }]),
                        format: Some("CD".to_string()),
                        format_id: None,
                        extra: HashMap::new(),
                    },
                ]),
                artist_credit: None,
                release_group: None,
                events: None,
                labels: None,
                asin: None,
                barcode: None,
                status: None,
                release_type: None,
                script: None,
                disambiguation: None,
                extra: HashMap::new(),
            }]),
            isrcs: None,
            tags: None,
            genres: None,
            disambiguation: None,
            first_release_date: None,
            extra: HashMap::new(),
        };

        let tags = recording_to_tags(&recording);

        // Total discs should be 2
        assert_eq!(tags.get("DiscTotal"), Some(&"2".to_string()));
    }
}
