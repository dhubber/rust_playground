use winit::event::ElementState::Pressed;
use rpgf::{Color4, Event, GameObject, Renderable, Transform2d};
use crate::START_POSITION;

pub const WALL_THICKNESS: f32 = 0.025;
pub const WALL_COLOR: [f32; 4] = [0.4, 0.5, 0.6, 1.0];

pub struct Wall {
    pub transform2d: Transform2d,
    pub renderable: Renderable,
}

impl Wall {
    pub fn new(position: [f32; 2], size: [f32; 2]) -> Self {
        Wall {
            transform2d: Transform2d {
                position,
                scale: size,
            },
            renderable: Renderable {
                color: WALL_COLOR,
            },
        }
    }
}

impl GameObject for Wall {
    fn transform2d(&self) -> &Transform2d {
        &self.transform2d
    }

    fn renderable(&self) -> &Renderable {
        &self.renderable
    }
}