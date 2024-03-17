use crate::{Camera2d, Event, EventListener, Scene};

pub trait Update {
    fn update(&mut self, time: f32, delta_time: f32) -> Option<Vec<Event>>;
}

pub trait SceneUpdate {
    fn update(&mut self, scene: &Scene, time: f32, delta_time: f32) -> Option<Vec<Event>>;
}

pub trait Game: Update + EventListener {
    fn new() -> Self;
    fn create_scene(&mut self) -> Scene;
    fn camera(&self) -> Camera2d;
    fn player_id(&self) -> u128;
}