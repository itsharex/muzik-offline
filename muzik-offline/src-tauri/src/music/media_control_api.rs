use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, PlatformConfig, MediaPlayback}; //, MediaPosition};
use tauri::{State, Window};
use std::{sync::{Arc, Mutex}, time::Duration};
use crate::{components::{audio_manager::SharedAudioManager, event_payload::Payload, song::Song}, database::db_manager::DbManager, utils::general_utils::get_song_cover_as_bytes};

pub fn config_mca(window: &Window) -> Option<MediaControls>{
    #[cfg(not(target_os = "windows"))]
    let hwnd: Option<*mut c_void> = None;

    // map(|handle| handle as *mut std::os::raw::c_void)
    #[cfg(target_os = "windows")]
    let hwnd: Option<*mut std::os::raw::c_void> = match window.hwnd() {
        Ok(handle) => Some(handle.0 as *mut std::os::raw::c_void),
        Err(_) => None,
    };

    let config = PlatformConfig {
        dbus_name: "muzik-offline",
        display_name: "muzik-offline",
        hwnd,
    };

    match MediaControls::new(config){
        Ok(cntrl) =>{
            return Some(cntrl);
        }
        Err(err) => {
            println!("Failed to initialize media controls: {:?}", err);
            return None;
        }
    };
}

pub fn configure_media_controls(controls: &mut MediaControls, cover_url: &str){
    match controls.set_metadata(MediaMetadata {
        title: Some("No song is playing"),
        artist: Some("No artist"),
        album: Some("No album"),
        cover_url: Some(cover_url),
        duration: None,
    }){
        Ok(_) => {},
        Err(_) => {},
    }
}

pub fn event_handler(window: &Window, event: &MediaControlEvent){
    match event {
        MediaControlEvent::Play => {
            // emit play event to window but don't crash/expect error if it fails we will just ignore the event request
            match window.emit("os-media-controls", Payload::new(MediaControlEvent::Play, None, None, None, None)){
                Ok(_) => {},
                Err(_) => {},
            }
        },
        MediaControlEvent::Pause => {
            // emit pause event to window but don't crash/expect error if it fails we will just ignore the event request
            match window.emit("os-media-controls", Payload::new(MediaControlEvent::Pause, None, None, None, None)){
                Ok(_) => {},
                Err(_) => {},
            }
        },
        MediaControlEvent::Next => {
            // emit next event to window but don't crash/expect error if it fails we will just ignore the event request
            match window.emit("os-media-controls", Payload::new(MediaControlEvent::Next, None, None, None, None)){
                Ok(_) => {},
                Err(_) => {},
            }
        },
        MediaControlEvent::Previous => {
            // emit previous event to window but don't crash/expect error if it fails we will just ignore the event request
            match window.emit("os-media-controls", Payload::new(MediaControlEvent::Previous, None, None, None, None)){
                Ok(_) => {},
                Err(_) => {},
            }
        },
        MediaControlEvent::Stop => {
            // emit stop event to window but don't crash/expect error if it fails we will just ignore the event request
            match window.emit("os-media-controls", Payload::new(MediaControlEvent::Stop, None, None, None, None)){
                Ok(_) => {},
                Err(_) => {},
            }
        },
        MediaControlEvent::Seek(sd) => {
            // emit seek event to window but don't crash/expect error if it fails we will just ignore the event request
            match window.emit("os-media-controls", Payload::new(MediaControlEvent::Seek(*sd), Some(*sd), None, None, None)){
                Ok(_) => {},
                Err(_) => {},
            }
        },
        MediaControlEvent::SeekBy(sd, duration) => {
            // emit seek by event to window but don't crash/expect error if it fails we will just ignore the event request
            match window.emit("os-media-controls", Payload::new(MediaControlEvent::SeekBy(*sd, *duration), Some(*sd), Some(duration.as_secs()), None, None)){
                Ok(_) => {},
                Err(_) => {},
            }
        },
        MediaControlEvent::SetPosition(mp) => {
            // emit set position event to window but don't crash/expect error if it fails we will just ignore the event request
            match window.emit("os-media-controls", Payload::new(MediaControlEvent::SetPosition(*mp), None, Some(mp.0.as_secs()), None, None)){
                Ok(_) => {},
                Err(_) => {},
            }
        },
        MediaControlEvent::SetVolume(volume) => {
            // emit set volume event to window but don't crash/expect error if it fails we will just ignore the event request
            match window.emit("os-media-controls", Payload::new(MediaControlEvent::SetVolume(*volume), None, None, Some(*volume), None)){
                Ok(_) => {},
                Err(_) => {},
            }
        },
        MediaControlEvent::OpenUri(uri) => {
            // emit open uri event to window but don't crash/expect error if it fails we will just ignore the event request
            match window.emit("os-media-controls", Payload::new(MediaControlEvent::OpenUri(uri.clone()), None, None, None, Some(uri.clone()))){
                Ok(_) => {},
                Err(_) => {},
            }
        },
        MediaControlEvent::Raise => {
            // emit raise event to window but don't crash/expect error if it fails we will just ignore the event request
            match window.emit("os-media-controls", Payload::new(MediaControlEvent::Raise, None, None, None, None)){
                Ok(_) => {},
                Err(_) => {},
            }
        },
        MediaControlEvent::Quit => {
            // emit quit event to window but don't crash/expect error if it fails we will just ignore the event request
            match window.emit("os-media-controls", Payload::new(MediaControlEvent::Quit, None, None, None, None)){
                Ok(_) => {},
                Err(_) => {},
            }
        },
        _ => {}
    }
}

#[tauri::command]
pub fn update_metadata(audio_manager: State<'_, Arc<Mutex<SharedAudioManager>>>, key: i32){
    let song: Song;

    match DbManager::new(){
        Ok(dbm) => {
            match dbm.song_tree.get(key.to_ne_bytes()){
                Ok(Some(song_as_ivec)) => {
                    let song_as_bytes = song_as_ivec.as_ref();
                    let song_as_str = String::from_utf8_lossy(song_as_bytes);
                    match serde_json::from_str::<Song>(&song_as_str.to_string()){
                        Ok(value) => {
                            song = value;
                        },
                        Err(_) => {
                            return;
                        },
                    }
                },
                Ok(None) => {
                    return;
                },
                Err(_) => {
                    return;
                },
            }
        }
        Err(_) => {
            return;
        }
    }

    let image_data: Vec<u8> = get_song_cover_as_bytes(&song, key);

    match audio_manager.lock(){
        Ok(mut manager) => {
            let cover_url = manager.cover_url.clone();
            manager.cover = image_data;
            match &mut manager.controls{
                Some(controller) => {
                    match controller.set_metadata(MediaMetadata {
                        title: Some(&song.name),
                        artist: Some(&song.artist),
                        album: Some(&song.album),
                        duration: Some(Duration::from_secs(song.duration_seconds)),
                        cover_url: Some(&cover_url),
                    }){
                        Ok(_) => {
        
                        }
                        Err(_) => {
        
                        }
                    }
                },
                None => {
        
                },
            }
        },
        Err(_) => {

        },
    }
}

#[tauri::command]
pub fn set_player_state(audio_manager: State<'_, Arc<Mutex<SharedAudioManager>>>, state: &str){//,position: f64
    match audio_manager.lock(){
        Ok(mut manager) => {
            let cover_url = manager.cover_url.clone();
            match &mut manager.controls{
                Some(controller) => {
                    match state{
                        "playing" => {
                            match controller.set_playback(MediaPlayback::Playing{progress: None }){//Some(MediaPosition(Duration::from_secs_f64(position)))}){
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        },
                        "paused" => {
                            match controller.set_playback(MediaPlayback::Paused{progress: None }){//Some(MediaPosition(Duration::from_secs_f64(position)))}){
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        },
                        "stopped" => {
                            match controller.set_playback(MediaPlayback::Stopped){
                                Ok(_) => {
                                    configure_media_controls(controller, &cover_url);
                                },
                                Err(_) => {},
                            }
                        },
                        _ => {},
                    }
                },
                None => {},
            }
        },
        Err(_) => {},
    }
}