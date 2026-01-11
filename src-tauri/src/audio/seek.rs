use crate::audio::shared::{DecoderCommand, PlaybackState};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub fn seek(
    state: &Arc<Mutex<PlaybackState>>,
    decoder_command_sender: &mut Option<Sender<DecoderCommand>>,
    position_seconds: f64,
) {
    println!("Seeking to: {}s", position_seconds);
    if let Some(decoder_command_sender) = &decoder_command_sender {
        let state = state.lock().unwrap();
        let sample_rate = state.sample_rate;
        drop(state);

        let target_samples = (position_seconds * sample_rate as f64) as u64;

        let _ = decoder_command_sender.send(DecoderCommand::Seek(target_samples));
        println!(
            "Seeking to {:.2} seconds ({} samples)",
            position_seconds, target_samples
        );
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;
    use super::*;

    #[test]
    fn test_seek_with_valid_position_and_active_decoder() {
        // Create a channel for decoder commands
        let (sender, receiver) = channel::<DecoderCommand>();

        // Create playback state with known sample rate (48000 Hz)
        let state = Arc::new(Mutex::new(PlaybackState {
            is_playing: true,
            is_paused: false,
            volume: 1.0,
            current_position_samples: 0,
            sample_rate: 48000,
            needs_buffer_clear: false,
        }));

        // Create a mutable option containing the sender
        let mut decoder_command_sender = Some(sender);

        // Seek to 5 seconds
        seek(&state, &mut decoder_command_sender, 5.0);

        // Receive the command and verify
        let command = receiver.recv().expect("Should receive seek command");

        // Expected samples: 5.0 * 48000 = 240000
        match command {
            DecoderCommand::Seek(samples) => {
                assert_eq!(
                    samples, 240000,
                    "Seek command should contain correct sample count"
                );
            }
            _ => panic!("Expected DecoderCommand::Seek, got different command"),
        }
    }
}
