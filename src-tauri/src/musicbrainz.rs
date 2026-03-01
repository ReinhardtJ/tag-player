use crate::read_music_library::Song;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Recording {
    pub id: String,
    pub title: String,
    pub length: Option<u32>,
    pub artist_credit: Option<Vec<ArtistCredit>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArtistCredit {
    pub name: String,
    pub artist: Artist,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
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
        "https://musicbrainz.org/ws/2/recording?query={}&fmt=json&limit=10",
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

    #[tokio::test]
    async fn test_search_song_on_musicbrainz_smells_like_teen_spirit() {
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

        assert!(result.is_ok(), "Search should succeed: {:?}", result);
        let recordings = result.unwrap();
        assert!(!recordings.is_empty(), "Should find at least one recording");
        
        let first = &recordings[0];
        assert_eq!(first.title, "Smells Like Teen Spirit");
    }
}
