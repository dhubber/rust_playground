use std::collections::HashMap;
use rand::prelude::*;
use crate::{Event, EventListener, GameObject};

pub struct Scene {
    pub name: String,
    pub objects: HashMap<u128,Box<dyn GameObject>>
}

impl Scene {
    pub fn new(name: String) -> Scene {
        Scene {
            name,
            objects: HashMap::new(),
        }
    }

    pub fn add_to_scene(&mut self, object: Box<dyn GameObject>) -> u128 {
        let id: u128 = self.generate_id();
        object.set_id(id);
        self.objects.insert(id, object);
        id
    }

    pub fn find_object(&self, id: &u128) -> Option<&Box<dyn GameObject>> {
        self.objects.get(id)
    }

    pub fn find_object_mut(&mut self, id: &u128) -> Option<&mut Box<dyn GameObject>> {
        self.objects.get_mut(id)
    }

    pub fn update(&mut self, time: f32, delta: f32) {
        for object in self.objects.values_mut() {
            if object.is_active() {
                object.update(time, delta);
            }
        }
    }

    fn generate_id(&self) -> u128 {
        let mut rng = rand::thread_rng();
        rng.gen()
    }
}

impl EventListener for Scene {
    fn on_event(&mut self, event: &Event) -> Option<Vec<Event>> {
        let object = self.find_object_mut(&event.id);
        match object {
            None => { None }
            Some(obj) => {
                obj.on_event(event)
            }
        }
    }
}