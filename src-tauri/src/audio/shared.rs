pub enum AudioPlayerCommand {
    LoadAndPlay(String), // path to audio file
    TogglePlayback,
    VolumeChange(f32), // volume between 0.0 and 1.0
    Seek(f64),         // position in seconds
}

pub enum DecoderCommand {
    Seek(u64), // seek to sample position
    Stop,
}

pub struct PlaybackState {
    pub is_playing: bool, // whether we have an audio file loaded
    pub is_paused: bool,  // whether the current audio file is paused
    pub volume: f32, // volume between 0.0 and 1.0
    pub current_position_samples: u64,
    pub sample_rate: u32,
    pub needs_buffer_clear: bool, // buffer needs to be cleared after seeking
}

