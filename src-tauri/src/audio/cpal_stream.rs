use std::sync::{Arc, Mutex};
use cpal::{default_host, Device, FromSample, OutputCallbackInfo, Sample, SampleFormat, SizedSample, Stream, StreamConfig};
use anyhow::{anyhow, Context, Error};
use ringbuf::HeapCons;
use cpal::traits::{DeviceTrait, HostTrait};
use ringbuf::consumer::Consumer;
use crate::audio::shared::PlaybackState;

pub fn create_audio_output(
    state: Arc<Mutex<PlaybackState>>,
    consumer: HeapCons<f32>,
    sample_rate: u32,
    channels: u16,
) -> Result<Stream, Error> {
    let device = get_default_audio_device()?;

    let default_config = device.default_output_config()?;

    // Use the audio file's sample rate and channel count, not the device's
    let config = StreamConfig {
        channels,
        sample_rate,
        buffer_size: cpal::BufferSize::Default,
    };

    println!("Using stream config: {:?}", config);

    let stream = match default_config.sample_format() {
        SampleFormat::F32 => build_stream::<f32>(&device, &config, state, consumer)?,  // No .into()
        SampleFormat::I16 => build_stream::<i16>(&device, &config, state, consumer)?,
        SampleFormat::U16 => build_stream::<u16>(&device, &config, state, consumer)?,
        _ => return Err(anyhow!("Unsupported sample format"))
    };

    Ok(stream)
}

fn get_default_audio_device() -> Result<Device, Error> {
    let host = default_host();
    let device = host
        .default_output_device()
        .context("No output device available")?;

    println!("Output device: {}", device.description()?);

    Ok(device)
}

fn build_stream<T>(
    device: &Device,
    config: &StreamConfig,
    state: Arc<Mutex<PlaybackState>>,
    mut consumer: HeapCons<f32>,
) -> Result<Stream, Error> where
    T: Sample + SizedSample + FromSample<f32>,
{
    let channels = config.channels as usize;

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], oci: &OutputCallbackInfo| audio_callback(
            data, oci, &state, &mut consumer, channels,
        ),
        |err| eprintln!("Audio stream error: {}", err),
        None,
    )?;

    Ok(stream)
}

fn audio_callback<T>(
    data: &mut [T],
    _: &OutputCallbackInfo,
    state: &Arc<Mutex<PlaybackState>>,
    consumer: &mut HeapCons<f32>,
    channels: usize,
) where
    T: Sample + SizedSample + FromSample<f32>,
{
    // state scope for getting volume
    let volume = {
        let state_guard = state.lock().unwrap();

        // if paused or not playing, output silence
        if !state_guard.is_playing || state_guard.is_paused {
            for sample in data.iter_mut() {
                *sample = Sample::EQUILIBRIUM;
            }
            return;
        }
        state_guard.volume
    };

    // read from ring buffer
    let mut temp_buffer = vec![0.0f32; data.len()];
    let samples_read = consumer.pop_slice(&mut temp_buffer);

    // apply volume and convert to output format
    for (i, sample) in temp_buffer[..samples_read].iter().enumerate() {
        data[i] = T::from_sample(sample * volume);
    }

    // fill remainder with silence if underrun
    for sample in data[samples_read..].iter_mut() {
        *sample = Sample::EQUILIBRIUM;
    }

    // scope for writing to state
    {
        // update position (approximate based on samples consumed)
        let mut state_guard = state.lock().unwrap();
        state_guard.current_position_samples += (samples_read / channels) as u64;
    }
}