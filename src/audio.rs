use std::collections::HashMap;
use std::path::Path;
use kira::{
    manager::{
        AudioManager, AudioManagerSettings,
        backend::DefaultBackend
    },
    sound::static_sound::{StaticSoundData, StaticSoundSettings}
};
use crate::{Event, EventListener, EventType};

pub struct AudioSystem {
    manager: AudioManager,
    sounds: HashMap<String,StaticSoundData>
}

impl AudioSystem {
    pub fn new() -> Self {
        AudioSystem {
            manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap(),
            sounds: HashMap::new(),
        }
    }

    pub fn register_audio_file(&mut self, name: String, filename: &Path) {
        let sound_data = StaticSoundData::from_file(filename, StaticSoundSettings::default());
        match sound_data {
            Ok(sound) => {
                self.sounds.insert(name, sound);
            }
            Err(e) => {
                println!("Error reading audio file: {}", e);
            }
        }
    }

    pub fn play_sound(&mut self, name: &String) {
        let sound = self.sounds.get(name);
        match sound {
            None => {
                println!("Error: Could not find sound: {name}");
            }
            Some(s) => {
                self.manager.play(s.clone()).expect("TODO: panic message");
            }
        }
    }
}

impl EventListener for AudioSystem {
    fn on_event(&mut self, event: &Event) -> Option<Vec<Event>> {
        match &event.event_type {
            EventType::PlayAudio(name) => {
                self.play_sound(name);
            }
            _ => ()
        }
        None
    }
}