use crate::read_music_library::Song;
use reqwest::Client;
use serde::Deserialize;
pub use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Recording {
    pub id: String,
    pub title: String,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub releases: Option<Vec<Release>>,
    pub isrcs: Option<Vec<String>>,
    pub tags: Option<Vec<Tag>>,
    pub genres: Option<Vec<Genre>>,
    #[serde(default)]
    pub disambiguation: Option<String>,
    #[serde(default)]
    pub first_release_date: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArtistCredit {
    pub name: String,
    pub artist: Artist,
    pub joinphrase: Option<String>,
    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub sort_name: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Release {
    pub id: String,
    pub title: String,
    pub date: Option<String>,
    pub country: Option<String>,
    pub media: Option<Vec<ReleaseMedia>>,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub release_group: Option<ReleaseGroup>,
    pub events: Option<Vec<ReleaseEvent>>,
    pub labels: Option<Vec<Label>>,
    pub asin: Option<String>,
    pub barcode: Option<String>,
    pub status: Option<String>,
    pub release_type: Option<Vec<String>>,
    pub script: Option<String>,
    pub disambiguation: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseGroup {
    pub id: String,
    pub first_release_date: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseEvent {
    pub date: Option<String>,
    pub country: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Label {
    pub name: Option<String>,
    pub catalog_number: Option<String>,
    pub label_code: Option<u32>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseMedia {
    pub track_count: Option<u32>,
    pub position: Option<u32>,
    pub tracks: Option<Vec<Track>>,
    pub format: Option<String>,
    pub format_id: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Track {
    pub id: Option<String>,
    pub number: Option<String>,
    pub position: Option<u32>,
    pub length: Option<u32>,
    pub isrcs: Option<Vec<String>>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
    pub name: String,
    pub count: i32,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Genre {
    pub name: String,
    pub count: i32,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Deserialize)]
struct SearchResponse {
    recordings: Vec<Recording>,
}

pub async fn search_song_on_musicbrainz(song: &Song) -> Result<Vec<Recording>, String> {
    let client = Client::builder()
        .user_agent("tag-player/0.1.0")
        .build()
        .map_err(|e| e.to_string())?;

    let query = build_query(song);
        let url = format!(
            "https://musicbrainz.org/ws/2/recording?query={}&fmt=json&limit=10&inc=artist-credits+releases+artist-credits+release-groups+aliases+tags+genres+isrcs",
            query
        );

    let result: SearchResponse = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result.recordings)
}

fn build_query(song: &Song) -> String {
    let mut parts = Vec::new();

    if let Some(title) = song.tags.get("TrackTitle") {
        parts.push(format!("recording:\"{}\"", escape_query(title)));
    }

    if let Some(artist) = song.tags.get("TrackArtist") {
        parts.push(format!("artist:\"{}\"", escape_query(artist)));
    }

    if let Some(album) = song.tags.get("AlbumTitle") {
        parts.push(format!("release:\"{}\"", escape_query(album)));
    }

    parts.join(" AND ")
}

fn escape_query(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_search_song_on_musicbrainz_smells_like_teen_spirit() {
        sleep(Duration::from_secs_f64(1.1)).await;

        let mut tags = HashMap::new();
        tags.insert("TrackTitle".to_string(), "Smells Like Teen Spirit".to_string());
        tags.insert("TrackArtist".to_string(), "Nirvana".to_string());
        tags.insert("AlbumTitle".to_string(), "Nevermind".to_string());

        let song = Song {
            path: "/test/path.mp3".to_string(),
            name: "Smells Like Teen Spirit.mp3".to_string(),
            duration_millis: 301000,
            tags,
            cover_base64: None,
        };

        let result = search_song_on_musicbrainz(&song).await;

        if let Err(ref e) = result {
            println!("Error: {}", e);
            // Try to get more details
            let client = reqwest::Client::builder()
                .user_agent("tag-player/0.1.0")
                .build()
                .unwrap();
            let query = build_query(&song);
            let url = format!(
                "https://musicbrainz.org/ws/2/recording?query={}&fmt=json&limit=10&inc=artist-credits+releases+artist-credits+release-groups+aliases+tags+genres+isrcs",
                query
            );
            println!("URL: {}", url);
            let resp = client.get(&url).send().await.unwrap();
            let text = resp.text().await.unwrap();
            
            // Try to deserialize to see what fails
            let parsed: Result<SearchResponse, _> = serde_json::from_str(&text);
            if let Err(ref de) = parsed {
                println!("Deserialization error: {}", de);
            }
            
            println!("Response (first 1000 chars): {}", &text[..1000.min(text.len())]);
        }
        assert!(result.is_ok(), "Search should succeed: {:?}", result);
        let recordings = result.unwrap();
        assert!(!recordings.is_empty(), "Should find at least one recording");

        let first = &recordings[0];
        assert_eq!(first.title, "Smells Like Teen Spirit");

        // Debug: print the response to see what's actually returned
        println!("First recording: {:#?}", first);

        // Verify artist is returned
        assert!(
            first.artist_credit.is_some(),
            "Should have artist_credit, got: {:#?}", first.artist_credit
        );
        let artist_credit = first.artist_credit.as_ref().unwrap();
        assert!(!artist_credit.is_empty(), "Should have at least one artist");
        assert_eq!(artist_credit[0].artist.name, "Nirvana");

        // Verify album is returned
        assert!(first.releases.is_some(), "Should have releases");
        let releases = first.releases.as_ref().unwrap();
        assert!(!releases.is_empty(), "Should have at least one release");
        assert_eq!(releases[0].title, "Nevermind");
    }
}