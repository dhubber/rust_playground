use crate::{Event};

#[derive(Copy, Clone, Debug)]
pub struct Transform2d {
    pub position: [f32; 2],
    pub scale: [f32; 2],
}

#[derive(Copy, Clone, Debug)]
pub struct Renderable {
    pub color: [f32; 4]
}

pub trait GameObject {
    fn transform2d(&self) -> Transform2d;
    fn renderable(&self) -> Renderable;
    fn update(&mut self, _time: f32, _delta_time: f32) {}
    fn on_event(&mut self, _event: Event) {}
}