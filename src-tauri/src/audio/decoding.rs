use crate::audio::shared::{DecoderCommand, PlaybackState};
use anyhow::{Context, Error};
use ringbuf::producer::Producer;
use ringbuf::HeapProd;
use std::io::ErrorKind;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{Decoder, DecoderOptions};
use symphonia::core::errors::Error::IoError;
use symphonia::core::formats::{FormatReader, SeekMode, SeekTo};
use symphonia::core::probe::ProbeResult;
use symphonia::core::units::Time;
use symphonia::default::get_codecs;

pub fn decoder_thread(
    probe_result: ProbeResult,
    mut producer: HeapProd<f32>,
    state: Arc<Mutex<PlaybackState>>,
    decoder_command_receiver: Receiver<DecoderCommand>,
) -> Result<(), Error> {
    let mut format_reader = probe_result.format;

    let default_track = format_reader
        .default_track()
        .context("No default track found")?;

    let track_id = default_track.id;
    let sample_rate = default_track
        .codec_params
        .sample_rate
        .context("No sample rate found")?;

    let duration_samples = default_track.codec_params.n_frames;

    // update state with track info
    {
        let mut state = state.lock().unwrap();
        state.sample_rate = sample_rate;
        state.duration_samples = duration_samples;
        state.current_position_samples = 0;
        state.is_playing = true;
        state.is_paused = false;
        state.needs_buffer_clear = false;
    }

    println!(
        "Sample rate: {}, Duration: {:?} samples",
        sample_rate, duration_samples
    );

    // create decoder
    let mut decoder = get_codecs().make(&default_track.codec_params, &DecoderOptions::default())?;

    // decode loop
    loop {
        // check for decoder commands (non-blocking)
        if let Ok(cmd) = decoder_command_receiver.try_recv() {
            match cmd {
                DecoderCommand::Seek(target_samples) => decode_samples(
                    state.clone(),
                    &mut format_reader,
                    &mut decoder,
                    target_samples,
                    sample_rate,
                    track_id,
                ),
                DecoderCommand::Stop => break,
            }
        }
        // check if we should stop
        {
            let state = state.lock().unwrap();
            if !state.is_playing {
                break;
            }
        }
        // Get next packet
        let packet = match format_reader.next_packet() {
            Ok(packet) => packet,
            Err(IoError(e)) if e.kind() == ErrorKind::UnexpectedEof => {
                println!("End of stream");
                break;
            }
            Err(e) => {
                eprintln!("Error reading packet: {}", e);
                break;
            }
        };

        // skip packets for other tracks
        if packet.track_id() != track_id {
            continue;
        }

        // decode the packet
        let decoded = match decoder.decode(&packet) {
            Ok(decoded) => decoded,
            Err(e) => {
                eprintln!("Decode error: {}", e);
                continue;
            }
        };

        // convert samples to f32 interleaved
        let spec = *decoded.spec();
        let duration = decoded.capacity() as u64;

        // convert to f32 samples
        let mut sample_buf = SampleBuffer::<f32>::new(duration, spec);
        sample_buf.copy_interleaved_ref(decoded);

        let samples = sample_buf.samples();

        // Push to ring buffer (blocking if full)
        let mut written = 0;
        while written < samples.len() {
            // check if paused
            {
                let state = state.lock().unwrap();
                if state.is_paused {
                    // wait while paused
                    drop(state);
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
                if !state.is_playing {
                    return Ok(());
                }
            }

            written += producer.push_slice(&samples[written..]);

            if written < samples.len() {
                // buffer full, wait a bit
                thread::sleep(Duration::from_millis(5));
            }
        }
    }

    let mut state = state.lock().unwrap();
    state.is_playing = false;
    Ok(())
}

fn decode_samples(
    state: Arc<Mutex<PlaybackState>>,
    format_reader: &mut Box<dyn FormatReader>,
    decoder: &mut Box<dyn Decoder>,
    target_samples: u64,
    sample_rate: u32,
    track_id: u32,
) {
    let target_seconds = target_samples as f64 / sample_rate as f64;

    let seek_result = format_reader.seek(
        SeekMode::Accurate,
        SeekTo::Time {
            time: Time::from(target_seconds),
            track_id: Some(track_id),
        },
    );

    if let Ok(seeked_result) = seek_result {
        println!("Seeked to timestamp: {}", seeked_result.actual_ts);

        let mut state = state.lock().unwrap();
        state.current_position_samples = target_samples;
        state.needs_buffer_clear = true;

        decoder.reset();
    } else {
        eprintln!("Seek failed");
    }
}
