use std::sync::mpsc::Sender;

pub fn get_duration_seconds(duration_samples: Option<u64>, sample_rate: u32) -> Option<f64> {
    duration_samples.map(|d| d as f64 / sample_rate as f64)
}

pub enum AudioCommand {
    LoadAndPlay(String), // path to audio file
    TogglePlayback,
    VolumeChange(f32), // volume between 0.0 and 1.0
    Seek(f64),         // position in seconds
}

pub struct AudioPlayer {
    pub sender: Sender<AudioCommand>,
}

pub enum DecoderCommand {
    Seek(u64), // seek to sample position
    Stop,
}

#[derive(serde::Serialize, Clone)]
pub struct AudioPosition {
    pub position: f64,
    pub duration: f64,
}

pub struct PlaybackState {
    pub is_playing: bool,
    pub is_paused: bool,
    pub volume: f32,
    pub current_position_samples: u64, // current playback position in samples
    pub sample_rate: u32,
    pub duration_samples: Option<u64>,
}