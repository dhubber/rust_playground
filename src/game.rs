use crate::{Event, Scene};

pub trait Update {
    fn update(&mut self, time: f32, delta_time: f32);
}

pub trait SceneUpdate {
    fn update(&mut self, scene: &Scene, time: f32, delta_time: f32) -> Option<Vec<Event>>;
}

pub trait Game {
    fn setup(&mut self);
    fn scene(&self) -> &Scene;
    fn update(&mut self, time: f32, delta_time: f32);
    //fn on_event(&mut self, id: u128, event: Event);
    fn player_id(&self) -> u128;
    fn console_log(&self) {}
}