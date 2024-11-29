use crate::components::rodio_audio_manager::RodioManager;
use std::sync::{Arc, Mutex};
use tauri::State;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, DeviceTrait, Source};

pub fn load_and_play_song_from_path_rodio(
    audio_manager: State<'_, Arc<Mutex<RodioManager>>>,
    sound_path: &str,
    volume: f64,
) {
    match audio_manager.lock() {
        Ok(manager) => {
            let file = File::open(sound_path).expect("Failed to open audio file");
            let source = Decoder::new(BufReader::new(file)).expect("Failed to create decoder");
            match manager.sink.lock(){
                Ok(sink_guard) => {
                    if let Some(ref sink) = *sink_guard {
                        // Clear the sink and append the new audio file
                        sink.stop();
                        sink.clear();
                        sink.append(source.convert_samples::<f32>());
                        sink.set_volume(volume as f32); // the volume will always be between 0.0 and 1.0 so hopefully this is safe
                        sink.play();
                    } else {
                        // Handle the case where the sink is None
                    }
                }
                Err(_) => {

                }
            }
        }
        Err(_) => {
            //failed to lock audio manager
        }
    }
}

pub fn load_a_song_from_path_rodio(
    audio_manager: State<'_, Arc<Mutex<RodioManager>>>,
    sound_path: &str,
    volume: f64,
) {
    match audio_manager.lock() {
        Ok(manager) => {
            let file = File::open(sound_path).expect("Failed to open audio file");
            let source = Decoder::new(BufReader::new(file)).expect("Failed to create decoder");
            match manager.sink.lock(){
                Ok(sink_guard) => {
                    if let Some(ref sink) = *sink_guard {
                        // Clear the sink and append the new audio file
                        sink.stop();
                        sink.clear();
                        sink.append(source.convert_samples::<f32>());
                        sink.set_volume(volume as f32); // the volume will always be between 0.0 and 1.0 so hopefully this is safe
                    } else {
                        // Handle the case where the sink is None
                    }
                }
                Err(_) => {

                }
            }
        }
        Err(_) => {
            //failed to lock audio manager
        }
    }
}

pub fn pause_song_rodio(audio_manager: State<'_, Arc<Mutex<RodioManager>>>) {
    match audio_manager.lock() {
        Ok(manager) => {
            match manager.sink.lock(){
                Ok(sink_guard) => {
                    if let Some(ref sink) = *sink_guard {
                        sink.pause();
                    } else {
                        // Handle the case where the sink is None
                    }
                }
                Err(_) => {

                }
            }
        }
        Err(_) => {
            //failed to lock audio manager
        }
    }
}

pub fn stop_song_rodio(audio_manager: State<'_, Arc<Mutex<RodioManager>>>) {
    match audio_manager.lock() {
        Ok(manager) => {
            match manager.sink.lock(){
                Ok(sink_guard) => {
                    if let Some(ref sink) = *sink_guard {
                        sink.stop();
                        sink.clear();
                    } else {
                        // Handle the case where the sink is None
                    }
                }
                Err(_) => {

                }
            }
        }
        Err(_) => {
            //failed to lock audio manager
        }
    }
}

pub fn resume_playing_rodio(audio_manager: State<'_, Arc<Mutex<RodioManager>>>) {
    match audio_manager.lock() {
        Ok(manager) => {
            match manager.sink.lock(){
                Ok(sink_guard) => {
                    if let Some(ref sink) = *sink_guard {
                        sink.play();
                    } else {
                        // Handle the case where the sink is None
                    }
                }
                Err(_) => {

                }
            }
        }
        Err(_) => {
            //failed to lock audio manager
        }
    }
}

pub fn seek_to_rodio(audio_manager: State<'_, Arc<Mutex<RodioManager>>>, position: f64) {
    let current_position = get_song_position_rodio(audio_manager.clone());

    if current_position > position {
        seek_by_rodio(audio_manager.clone(), -1.0 * (current_position - position));
    } else {
        seek_by_rodio(audio_manager.clone(), position - current_position);
    }

}

pub fn seek_by_rodio(audio_manager: State<'_, Arc<Mutex<RodioManager>>>, delta: f64) {
    match audio_manager.lock() {
        Ok(manager) => {
            match manager.sink.lock(){
                Ok(sink_guard) => {
                    if let Some(ref sink) = *sink_guard {
                        match sink.try_seek(std::time::Duration::from_secs_f64(delta)){
                            Ok(_) => {

                            }
                            Err(_) => {
                                //failed to seek
                            }
                        }
                    } else {
                        // Handle the case where the sink is None
                    }
                }
                Err(_) => {

                }
            }
        }
        Err(_) => {
            //failed to lock audio manager
        }
    }
}

pub fn get_song_position_rodio(audio_manager: State<'_, Arc<Mutex<RodioManager>>>) -> f64 {
    match audio_manager.lock() {
        Ok(manager) => {
            match manager.sink.lock(){
                Ok(sink_guard) => {
                    if let Some(ref sink) = *sink_guard {
                        sink.get_pos().as_secs_f64()
                    } else {
                        // Handle the case where the sink is None
                        0.0
                    }
                }
                Err(_) => {
                    0.0
                }
            }
        }
        Err(_) => {
            //failed to lock audio manager
            0.0
        }
    }
}

pub fn set_volume_rodio(audio_manager: State<'_, Arc<Mutex<RodioManager>>>, volume: f64) {
    match audio_manager.lock() {
        Ok(manager) => {
            match manager.sink.lock(){
                Ok(sink_guard) => {
                    if let Some(ref sink) = *sink_guard {
                        sink.set_volume(volume as f32); // the volume will always be between 0.0 and 1.0 so hopefully this is safe
                    } else {
                        // Handle the case where the sink is None
                    }
                }
                Err(_) => {

                }
            }
        }
        Err(_) => {
            //failed to lock audio manager
        }
    }
}

#[tauri::command]
pub fn get_output_devices() -> Vec<String> {
    use rodio::cpal::traits::HostTrait;
    let host = rodio::cpal::default_host();
    let devices: Vec<rodio::Device> = match host.output_devices(){
        Ok(devices) => {
            devices.collect()
        }
        Err(_) => {
            return Vec::new();
        }
    };

    let mut usable_device_names = Vec::new();

    for device in &devices{
        match device.name(){
            Ok(name) => {
                usable_device_names.push(name);
            }
            Err(_) => {

            }
        }
    }
    
    usable_device_names
}

#[tauri::command]
pub fn set_output_device(
    audio_manager: State<'_, Arc<Mutex<RodioManager>>>,
    device_name: &str
) {
    use rodio::cpal::traits::HostTrait;
    let host = rodio::cpal::default_host();
    let devices: Vec<rodio::Device> = match host.output_devices(){
        Ok(devices) => {
            devices.collect()
        }
        Err(_) => {
            return;
        }
    };

    let device = devices.iter().find(|device| {
        match device.name(){
            Ok(name) => {
                name == device_name
            }
            Err(_) => {
                false
            }
        }
    });

    match audio_manager.lock() {
        Ok(manager) => {
            match device{
                Some(device) => {
                    manager.set_device(device.clone());
                }
                None => {
                    // Handle the case where the device was not found
                }
            }
        }
        Err(_) => {
            //failed to lock audio manager
        }
    }
}