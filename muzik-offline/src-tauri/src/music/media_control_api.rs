use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, PlatformConfig};
use tauri::{State, Window};
use std::sync::Mutex;
use crate::components::audio_manager::SharedAudioManager;

pub fn config_mca(window: &Window) -> Option<MediaControls>{
    #[cfg(not(target_os = "windows"))]
    let hwnd: Option<*mut c_void> = None;

    #[cfg(target_os = "windows")]
    let hwnd: Option<*mut std::ffi::c_void> = {
        let pointer = Box::new(window);
        Some(Box::into_raw(pointer) as *mut std::ffi::c_void)
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
        Err(_) => {
            return None;
        }
    };
}

pub fn event_handler(window: &Window, event: &MediaControlEvent){
    match event {
        MediaControlEvent::Play => {
            // emit play event to window
            window.emit("os-media-controls", String::from("Play")).expect("failed to emit play event");
        },
        MediaControlEvent::Pause => {
            // emit pause event to window
            window.emit("os-media-controls", String::from("Pause")).expect("failed to emit pause event");
        },
        MediaControlEvent::Next => {
            // emit next event to window
            window.emit("os-media-controls", String::from("Next")).expect("failed to emit next event");
        },
        MediaControlEvent::Previous => {
            // emit previous event to window
            window.emit("os-media-controls", String::from("Previous")).expect("failed to emit previous event");
        },
        MediaControlEvent::Stop => {
            // emit stop event to window
            window.emit("os-media-controls", String::from("Stop")).expect("failed to emit stop event");
        },
        MediaControlEvent::Seek(sd) => {
            // emit seek event to window
            window.emit("os-media-controls", String::from("Seek")).expect("failed to emit seek event");
        },
        MediaControlEvent::SeekBy(sd, duration) => {
            // emit seek by event to window
            window.emit("os-media-controls", String::from("SeekBy")).expect("failed to emit seek by event");
        },
        MediaControlEvent::SetPosition(mp) => {
            // emit set position event to window
            window.emit("os-media-controls", String::from("SetPosition")).expect("failed to emit set position event");
        },
        MediaControlEvent::SetVolume(volume) => {
            // emit set volume event to window
            window.emit("os-media-controls", String::from("SetVolume")).expect("failed to emit set volume event");
        },
        MediaControlEvent::OpenUri(uri) => {
            // emit open uri event to window
            window.emit("os-media-controls", String::from("OpenUri")).expect("failed to emit open uri event");
        },
        MediaControlEvent::Raise => {
            // emit raise event to window
            window.emit("os-media-controls", String::from("Raise")).expect("failed to emit raise event");
        },
        MediaControlEvent::Quit => {
            // emit quit event to window
            window.emit("os-media-controls", String::from("Quit")).expect("failed to emit quit event");
        },
        _ => {}
    }
}

#[tauri::command]
pub fn update_metadata(audio_manager: State<'_, Mutex<SharedAudioManager>>, title: &str, artist: &str, album: &str){
    match audio_manager.lock(){
        Ok(mut manager) => {
            match &mut manager.controls{
                Some(controller) => {
                    match controller.set_metadata(MediaMetadata {
                        title: Some(title),
                        artist: Some(artist),
                        album: Some(album),
                        ..Default::default()
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