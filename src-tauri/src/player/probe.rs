use anyhow::Error;
use std::fs::File;
use std::path::Path;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::{Hint, ProbeResult};
use symphonia::default::get_probe;

pub fn probe_audio_file(path: &str) -> Result<ProbeResult, Error> {
    // open the file
    let file = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    // create a hint for the probe based on the file extension
    let mut hint = Hint::new();
    if let Some(ext) = Path::new(&path).extension() {
        if let Some(ext_str) = ext.to_str() {
            hint.with_extension(ext_str);
        }
    }

    // probe the media source for format
    let probed = get_probe().format(
        &hint,
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;
    Ok(probed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe_mp3_file() {
        let path = "./tests/music_libraries/different_formats/some_song.mp3";
        let result = probe_audio_file(path);

        assert!(result.is_ok(), "Failed to probe MP3 file");

        let probed = result.unwrap();
        assert!(
            probed.format.default_track().is_some(),
            "No default track found in MP3"
        );

        let track = probed.format.default_track().unwrap();
        assert!(
            track.codec_params.sample_rate.is_some(),
            "No sample rate in MP3"
        );

        println!(
            "MP3 file probed successfully: {} Hz, {} channels",
            track.codec_params.sample_rate.unwrap(),
            track.codec_params.channels.map(|c| c.count()).unwrap_or(2)
        );
    }

    #[test]
    fn test_probe_wav_file() {
        let path = "./tests/music_libraries/different_formats/some_song.wav";
        let result = probe_audio_file(path);

        assert!(result.is_ok(), "Failed to probe WAV file");

        let probed = result.unwrap();
        assert!(
            probed.format.default_track().is_some(),
            "No default track found in WAV"
        );

        let track = probed.format.default_track().unwrap();
        assert!(
            track.codec_params.sample_rate.is_some(),
            "No sample rate in WAV"
        );

        println!(
            "WAV file probed successfully: {} Hz, {} channels",
            track.codec_params.sample_rate.unwrap(),
            track.codec_params.channels.map(|c| c.count()).unwrap_or(2)
        );
    }

    #[test]
    fn test_probe_flac_file() {
        let path = "./tests/music_libraries/different_formats/some_audio.flac";
        let result = probe_audio_file(path);

        assert!(result.is_ok(), "Failed to probe FLAC file");

        let probed = result.unwrap();
        assert!(
            probed.format.default_track().is_some(),
            "No default track found in FLAC"
        );

        let track = probed.format.default_track().unwrap();
        assert!(
            track.codec_params.sample_rate.is_some(),
            "No sample rate in FLAC"
        );

        println!(
            "FLAC file probed successfully: {} Hz, {} channels",
            track.codec_params.sample_rate.unwrap(),
            track.codec_params.channels.map(|c| c.count()).unwrap_or(2)
        );
    }

    #[test]
    fn test_probe_nonexistent_file() {
        let path = "./tests/music_libraries/nonexistent.mp3";
        let result = probe_audio_file(path);

        assert!(result.is_err(), "Should fail to probe nonexistent file");
    }

    #[test]
    fn test_probe_invalid_file() {
        let path = "./tests/music_libraries/one_file_with_tags"; // directory, not a file
        let result = probe_audio_file(path);

        assert!(result.is_err(), "Should fail to probe directory as file");
    }
}
