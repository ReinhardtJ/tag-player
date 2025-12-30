use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{Receiver, Sender};
use rodio::Decoder;

pub enum AudioCommand {
    LoadAndPlay(String),
    TogglePlayback,
    VolumeChange(f32)
}

pub struct AudioPlayer {
    pub sender: Sender<AudioCommand>,
}

pub fn audio_thread(receiver: Receiver<AudioCommand>) {
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("Failed to create audio stream");

    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    loop {
        match receiver.recv() {
            Ok(AudioCommand::LoadAndPlay(path)) => {
                sink.stop();
                println!("Attempting to open file: {}", path);
                match File::open(&path) {
                    Ok(file) => {
                        let buf_reader = BufReader::new(file);
                        match Decoder::new(buf_reader) {
                            Ok(source) => {
                                sink.append(source);
                                sink.play();
                                println!("Playing: {}", path)
                            }
                            Err(e) => eprintln!("Failed to decode audio: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Failed to open file: {}", e),
                }
            }
            Ok(AudioCommand::TogglePlayback) => {
                if sink.is_paused() {
                    sink.play();
                    println!("Resumed playback")
                } else {
                    sink.pause();
                    println!("Paused playback")
                }
            }
            Ok(AudioCommand::VolumeChange(volume)) => {
                sink.set_volume(volume)
            }
            Err(_) => {
                println!("Audio thread shutting down");
                break;
            }
        }
    }
}