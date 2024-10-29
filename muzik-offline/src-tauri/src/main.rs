// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod commands;
mod music;
mod components;
mod utils;
mod database;
mod socials;
mod constants;

use constants::null_cover_null::NULL_COVER_NULL;
use kira::manager::{AudioManager, AudioManagerSettings, backend::DefaultBackend};
use music::media_control_api::configure_media_controls;
use souvlaki::{MediaControlEvent, MediaControls};
use components::audio_manager::SharedAudioManager;
use utils::general_utils::decode_image_in_parallel;
use utils::music_list_organizer::MLO;
use socials::discord_rpc::DiscordRpc;
use warp::{Filter, http::Uri, reply::Response, Reply};

use tauri::Manager;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tauri::async_runtime::{self, spawn};

use crate::app::controller::{toggle_app_pin, toggle_miniplayer_view, drag_app_window};
use crate::commands::metadata_retriever::get_all_songs;
use crate::commands::general_commands::{open_in_file_manager, resize_frontend_image_to_fixed_height, get_audio_dir};
use crate::music::player::{load_and_play_song_from_path, load_a_song_from_path, set_volume, pause_song, resume_playing, seek_to, seek_by, get_song_position, stop_song};
use crate::music::media_control_api::{config_mca, update_metadata, event_handler, set_player_state};
use crate::utils::music_list_organizer::{mlo_set_shuffle_list, mlo_set_repeat_list, mlo_get_next_batch_as_size, mlo_reset_and_set_remaining_keys};
use crate::utils::general_utils::get_random_port;
use crate::database::db_api::{get_batch_of_songs, get_batch_of_albums, get_batch_of_artists, get_batch_of_genres};
use crate::socials::discord_rpc::{allow_connection_and_connect_to_discord_rpc, attempt_to_connect_if_possible, disallow_connection_and_close_discord_rpc, set_discord_rpc_activity, clear_discord_rpc_activity};

fn main() {
    tauri::Builder::default()
        .manage(initialize_audio_manager())
        .manage(Mutex::new(MLO::new()))
        .manage(Mutex::new(DiscordRpc::new().expect("failed to initialize discord rpc")))
        .setup(setup_app)
        .invoke_handler(tauri::generate_handler![
            // WINDOW CONTROL
            toggle_app_pin,
            toggle_miniplayer_view,
            drag_app_window,
            update_metadata,
            set_player_state,

            // GENERAL COMMANDS
            get_all_songs,
            open_in_file_manager,
            set_volume,
            get_audio_dir,

            // MUSIC PLAYER
            load_and_play_song_from_path,
            load_a_song_from_path,
            pause_song,
            resume_playing,
            stop_song,
            seek_to,
            seek_by,
            get_song_position,

            // UTILS
            resize_frontend_image_to_fixed_height,

            // MUSIC LIST ORGANIZER
            mlo_set_shuffle_list,
            mlo_set_repeat_list,
            mlo_reset_and_set_remaining_keys,
            mlo_get_next_batch_as_size,

            // DATABASE API
            get_batch_of_songs,
            get_batch_of_albums,
            get_batch_of_artists,
            get_batch_of_genres,

            // DISCORD RPC
            allow_connection_and_connect_to_discord_rpc,
            attempt_to_connect_if_possible,
            disallow_connection_and_close_discord_rpc,
            set_discord_rpc_activity,
            clear_discord_rpc_activity
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Initializes the SharedAudioManager with required settings.
fn initialize_audio_manager() -> Arc<Mutex<SharedAudioManager>> {
    let audio_manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
        .expect("failed to initialize audio manager");

    Arc::new(Mutex::new(SharedAudioManager {
        manager: audio_manager,
        instance_handle: None,
        volume: 0.0,
        controls: None,
        cover: decode_image_in_parallel(&NULL_COVER_NULL.to_owned()).expect("failed to decode image"),
        cover_url: String::new(),
    }))
}

/// Sets up the Tauri application.
fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx): (mpsc::Sender<MediaControlEvent>, mpsc::Receiver<MediaControlEvent>) = mpsc::channel(32);
    let shared_audio_manager = Arc::clone(&app.state::<Arc<Mutex<SharedAudioManager>>>());
    let window = app.app_handle().get_window("main").expect("failed to get window");

    // Set up the image route
    let image_route = create_image_route(shared_audio_manager.clone());

    // get random port for warp server
    let port = get_random_port();

    // Start the warp server
    spawn({
        warp::serve(image_route)
            .run(([127, 0, 0, 1], port))
    });

    // add url to shared audio manager
    shared_audio_manager.lock().expect("failed to lock shared audio manager").cover_url = format!("http://localhost:{}/cover", port);

    // Set up media controls
    let mut controls = config_mca(&window).expect("Failed to initialize media controls");
    setup_media_controls(&mut controls, tx.clone(), port);

    // Store the controls in the shared audio manager
    shared_audio_manager.lock().expect("failed to lock shared audio manager").controls = Some(controls);

    // Handle media control events
    spawn(async move {
        while let Some(event) = rx.recv().await {
            event_handler(&window, &event);
        }
    });
    Ok(())
}

/// Creates the image route for serving the cover image.
fn create_image_route(shared_audio_manager: Arc<Mutex<SharedAudioManager>>) -> impl Filter<Extract = (Response,), Error = warp::Rejection> + Clone {
    warp::path("cover")
        .and(warp::get())
        .map(move || {
            match shared_audio_manager.lock() {
                Ok(audio_manager) => {
                    warp::reply::with_header(audio_manager.cover.clone(), "Content-Type", "image/png").into_response()
                },
                Err(_) => {
                    // Redirect to default image on Imgur
                    warp::redirect::temporary(Uri::from_static("https://i.imgur.com/1bJ0j6V.png")).into_response()
                }
            }
        })
}

/// Sets up the media controls for the application.
fn setup_media_controls(controls: &mut MediaControls, tx: mpsc::Sender<MediaControlEvent>, port: u16) {
    controls.attach(move |event: MediaControlEvent| {
        let tx = tx.clone();
        async_runtime::spawn(async move {
            if tx.send(event).await.is_err() {
                println!("Failed to send event");
            }
        });
    }).expect("Failed to attach media controls");

    configure_media_controls(controls, &format!("http://localhost:{}/cover", port).to_owned());
}
