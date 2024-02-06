use std::collections::HashMap;
use rand::prelude::*;
use crate::GameObject;

pub struct Scene {
    pub name: String,
    pub objects: HashMap<u128,GameObject>,
}

impl Scene {
    pub fn new(name: String) -> Self {
        Scene {
            name,
            objects: HashMap::new(),
        }
    }

    pub fn new_object(&mut self, position: [f32; 2], scale: [f32; 2], color: [f32; 4]) {
        let mut rng = rand::thread_rng();
        let id: u128 = rng.gen();
        let object = GameObject {
            id,
            position,
            scale,
            color
        };
        self.objects.insert(id, object);
    }

    pub fn find_object(&mut self, id: &u128) -> Option<&mut GameObject> {
        self.objects.get_mut(id)
    }
}