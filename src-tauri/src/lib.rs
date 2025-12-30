// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use tauri::State;

enum AudioCommand {
    LoadAndPlay(String),
    TogglePlayback,
}

struct AudioPlayer {
    sender: Sender<AudioCommand>,
}

#[tauri::command]
fn load_and_play(path: String, state: State<AudioPlayer>) -> Result<(), String> {
    state
        .sender
        .send(AudioCommand::LoadAndPlay(path))
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn toggle_playback(state: State<AudioPlayer>) -> Result<(), String> {
    state
        .sender
        .send(AudioCommand::TogglePlayback)
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn audio_thread(receiver: Receiver<AudioCommand>) {
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
            Err(_) => {
                println!("Audio thread shutting down");
                break;
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (sender, receiver) = channel();
    thread::spawn(move || audio_thread(receiver));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AudioPlayer { sender })
        .invoke_handler(tauri::generate_handler![load_and_play, toggle_playback])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
