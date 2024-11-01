use serde::{Serialize, Deserialize};
use souvlaki::{MediaControlEvent, SeekDirection};

#[derive(Clone, Serialize, Deserialize)]
pub struct Payload{
    pub event: String, 
    pub seek_direction: String,
    pub duration: Option<u64>,
    pub volume: Option<f64>,
    pub uri: Option<String>,
}

impl Payload{
    pub fn new(event: MediaControlEvent, sd: Option<SeekDirection>, duration: Option<u64>, volume: Option<f64>, uri: Option<String>) -> Self{
        let event = match event{
            MediaControlEvent::Play => "Play",
            MediaControlEvent::Pause => "Pause",
            MediaControlEvent::Toggle => "Toggle",
            MediaControlEvent::Next => "Next",
            MediaControlEvent::Previous => "Previous",
            MediaControlEvent::Stop => "Stop",
            MediaControlEvent::Seek(_) => "Seek",
            MediaControlEvent::SeekBy(_, _) => "SeekBy",
            MediaControlEvent::SetPosition(_) => "SetPosition",
            MediaControlEvent::SetVolume(_) => "SetVolume",
            MediaControlEvent::OpenUri(_) => "OpenUri",
            MediaControlEvent::Raise => "Raise",
            MediaControlEvent::Quit => "Quit",
        };

        let seek_direction = match sd{
            Some(SeekDirection::Forward) => "Forward",
            Some(SeekDirection::Backward) => "Backward",
            None => "",
        };

        Self{
            event: String::from(event),
            seek_direction: String::from(seek_direction),
            duration,
            volume,
            uri,
        }
    }
}