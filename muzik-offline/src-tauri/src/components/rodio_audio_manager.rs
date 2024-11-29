use rodio::{OutputStream, Device, Sink};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};

enum AudioCommand {
    SetDevice(Device)
}

pub struct RodioManager {
    pub sender: Sender<AudioCommand>,
    pub sink: Arc<Mutex<Option<Sink>>>,
}

impl RodioManager {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        let sink: Arc<Mutex<Option<Sink>>> = Arc::new(Mutex::new(None));
        let cloned_sink = sink.clone();

        std::thread::spawn(move || {
            // Initialize the default output stream
            let (mut _current_stream, mut _current_handle) = OutputStream::try_default().expect("No default output stream available");
            cloned_sink.lock().expect("unable to lock").replace(Sink::try_new(&_current_handle).expect("Failed to create sink"));

            for command in receiver {
                match command {
                    AudioCommand::SetDevice(device) => {
                        if let Ok((new_stream, new_handle)) = OutputStream::try_from_device(&device) {
                            _current_stream = new_stream;
                            _current_handle = new_handle;
                            println!("Switched audio device successfully!");
                        } else {
                            eprintln!("Failed to switch audio device");
                        }
                    }
                }
            }
        });

        Self { 
            sender,
            sink: sink.clone()
        }
    }

    pub fn set_device(&self, device: Device) {
        match self.sender.send(AudioCommand::SetDevice(device)){
            Ok(_) => {

            }
            Err(_) => {
                
            }
        }
    }
}