use crate::{Event};
use crate::aab::AxisAlignedBox;

#[derive(Copy, Clone, Debug)]
pub struct Transform2d {
    pub position: [f32; 2],
    pub scale: [f32; 2],
}

impl Transform2d {
    pub fn set_position(&mut self, new_position: [f32; 2]) {
        self.position = new_position;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Renderable {
    pub color: [f32; 4]
}

pub trait GameObject {
    fn is_dirty(&self) -> bool { false }
    fn transform2d(&self) -> &Transform2d;
    fn renderable(&self) -> &Renderable;
    fn aa_box(&self) -> AxisAlignedBox {
        let transform2d = self.transform2d();
        AxisAlignedBox {
            min: [transform2d.position[0] - 0.5*transform2d.scale[0], transform2d.position[1] - 0.5*transform2d.scale[1]],
            max: [transform2d.position[0] + 0.5*transform2d.scale[0], transform2d.position[1] + 0.5*transform2d.scale[1]],
        }
    }
    fn update(&mut self, _time: f32, _delta_time: f32) {}
    fn on_event(&mut self, _event: Event) {}
}