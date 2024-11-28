use crate::components::{rodio_audio_manager::RodioManager, kira_audio_manager::KiraManager};
use std::sync::{Arc, Mutex};
use tauri::State;

use super::{
    kira_player::{
        load_and_play_song_from_path_kira, 
        load_a_song_from_path_kira, 
        pause_song_kira,
        stop_song_kira,
        resume_playing_kira,
        seek_to_kira,
        seek_by_kira,
        get_song_position_kira,
        set_volume_kira
    }, 
    rodio_player::{
        load_and_play_song_from_path_rodio,
        load_a_song_from_path_rodio,
        pause_song_rodio,
        stop_song_rodio,
        resume_playing_rodio,
        seek_to_rodio,
        seek_by_rodio,
        get_song_position_rodio,
        set_volume_rodio
    }};

#[tauri::command]
pub fn load_and_play_song_from_path(
    rodio_audio_manager: State<'_, Arc<Mutex<RodioManager>>>,
    kira_audio_manager: State<'_, Arc<Mutex<KiraManager>>>,
    sound_path: &str,
    player: &str,
    volume: f64,
) {
    match player{
        "rodio" => {
            load_and_play_song_from_path_rodio(rodio_audio_manager, sound_path, volume);
        }
        "kira" => {
            load_and_play_song_from_path_kira(kira_audio_manager, sound_path, volume);
        }
        _ => {
            // Handle the case where the player is not recognized
        }
    }
}

#[tauri::command]
pub fn load_a_song_from_path(
    rodio_audio_manager: State<'_, Arc<Mutex<RodioManager>>>,
    kira_audio_manager: State<'_, Arc<Mutex<KiraManager>>>,
    sound_path: &str,
    player: &str,
    volume: f64,
) {
    match player{
        "rodio" => {
            load_a_song_from_path_rodio(rodio_audio_manager, sound_path, volume);
        }
        "kira" => {
            load_a_song_from_path_kira(kira_audio_manager, sound_path, volume);
        }
        _ => {
            // Handle the case where the player is not recognized
        }
    }
}

#[tauri::command]
pub fn pause_song(rodio_audio_manager: State<'_, Arc<Mutex<RodioManager>>>, kira_audio_manager: State<'_, Arc<Mutex<KiraManager>>>, player: &str) {
    match player{
        "rodio" => {
            pause_song_rodio(rodio_audio_manager);
        }
        "kira" => {
            pause_song_kira(kira_audio_manager);
        }
        _ => {
            // Handle the case where the player is not recognized
        }
    }
    
}

#[tauri::command]
pub fn stop_song(rodio_audio_manager: State<'_, Arc<Mutex<RodioManager>>>, kira_audio_manager: State<'_, Arc<Mutex<KiraManager>>>, player: &str) {
    match player{
        "rodio" => {
            stop_song_rodio(rodio_audio_manager);
        }
        "kira" => {
            stop_song_kira(kira_audio_manager);
        }
        _ => {
            // Handle the case where the player is not recognized
        }
    }
    
}

#[tauri::command]
pub fn resume_playing(rodio_audio_manager: State<'_, Arc<Mutex<RodioManager>>>, kira_audio_manager: State<'_, Arc<Mutex<KiraManager>>>, player: &str) {
    match player{
        "rodio" => {
            resume_playing_rodio(rodio_audio_manager);
        }
        "kira" => {
            resume_playing_kira(kira_audio_manager);
        }
        _ => {
            // Handle the case where the player is not recognized
        }
    }
    
}

#[tauri::command]
pub fn seek_to(
    rodio_audio_manager: State<'_, Arc<Mutex<RodioManager>>>,
    kira_audio_manager: State<'_, Arc<Mutex<KiraManager>>>,
    player: &str,
    position: f64
) {
    match player{
        "rodio" => {
            seek_to_rodio(rodio_audio_manager, position);
        }
        "kira" => {
            seek_to_kira(kira_audio_manager, position);
        }
        _ => {
            // Handle the case where the player is not recognized
        }
    }
}

#[tauri::command]
pub fn seek_by(
    rodio_audio_manager: State<'_, Arc<Mutex<RodioManager>>>,
    kira_audio_manager: State<'_, Arc<Mutex<KiraManager>>>,
    player: &str,
    delta: f64
) {
    match player{
        "rodio" => {
            seek_by_rodio(rodio_audio_manager, delta);
        }
        "kira" => {
            seek_by_kira(kira_audio_manager, delta);
        }
        _ => {
            // Handle the case where the player is not recognized
        }
    }
}

#[tauri::command]
pub fn get_song_position(
    rodio_audio_manager: State<'_, Arc<Mutex<RodioManager>>>,
    kira_audio_manager: State<'_, Arc<Mutex<KiraManager>>>,
    player: &str
) -> f64 {
    match player{
        "rodio" => {
            get_song_position_rodio(rodio_audio_manager)
        }
        "kira" => {
            get_song_position_kira(kira_audio_manager)
        }
        _ => {
            // Handle the case where the player is not recognized
            0.0
        }
    }
}

#[tauri::command]
pub fn set_volume(
    rodio_audio_manager: State<'_, Arc<Mutex<RodioManager>>>,
    kira_audio_manager: State<'_, Arc<Mutex<KiraManager>>>,
    player: &str,
    volume: f64
) {
    match player{
        "rodio" => {
            set_volume_rodio(rodio_audio_manager, volume);
        }
        "kira" => {
            set_volume_kira(kira_audio_manager, volume);
        }
        _ => {
            // Handle the case where the player is not recognized
        }
    }
}