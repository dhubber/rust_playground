use crate::{Event, EventListener, Scene};

pub trait Update {
    fn update(&mut self, time: f32, delta_time: f32) -> Option<Vec<Event>>;
}

pub trait SceneUpdate {
    fn update(&mut self, scene: &Scene, time: f32, delta_time: f32) -> Option<Vec<Event>>;
}

/*pub trait GameLogic {
    fn new() -> Self;
    //fn setup(&mut self);
    fn create_scene(&mut self) -> Scene;
    //fn scene(&self) -> &Scene;
    fn update(&mut self, time: f32, delta_time: f32) -> Option<Vec<Event>>;
    //fn on_event(&mut self, id: u128, event: Event) -> Option<Vec<Event>>;
    fn player_id(&self) -> u128;
    fn console_log(&self) {}
}*/

pub trait Game: Update + EventListener {
    fn new() -> Self;
    fn setup(&mut self);
    fn scene(&self) -> &Scene;
    //fn update(&mut self, time: f32, delta_time: f32) -> Option<Vec<Event>>;
    //fn on_event(&mut self, id: u128, event: Event) -> Option<Vec<Event>>;
    fn player_id(&self) -> u128;
    //fn console_log(&self) {}
}