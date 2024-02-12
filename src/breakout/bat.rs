use winit::event::ElementState::Pressed;
use rpgf::{Event, GameObject, Renderable, Transform2d};
use crate::{LEVEL_WIDTH, START_POSITION};
use crate::wall::WALL_THICKNESS;

const BAT_SPEED: f32 = 1.0;
pub const BAT_SIZE: [f32; 2] = [0.1, 0.025];
const BAT_COLOR: [f32; 4] = [0.8, 0.75, 0.9, 1.0];

pub struct Bat {
    pub transform2d: Transform2d,
    pub renderable: Renderable,
    movement_range: [f32; 2],
    speed: f32,
    left_pressed: bool,
    right_pressed: bool,
    dirty: bool
}

impl Bat {
    pub fn new() -> Self {
        Bat {
            transform2d: Transform2d {
                position: START_POSITION,
                scale: BAT_SIZE,
            },
            renderable: Renderable {
                color: BAT_COLOR,
            },
            movement_range: [-0.5*LEVEL_WIDTH + WALL_THICKNESS + 0.5*BAT_SIZE[0],
                0.5*LEVEL_WIDTH - WALL_THICKNESS - 0.5*BAT_SIZE[0]],
            speed: BAT_SPEED,
            left_pressed: false,
            right_pressed: false,
            dirty: false,
        }
    }

    pub fn try_move_x(&mut self, delta_x: f32) {
        let new_x = (self.transform2d.position[0] + delta_x)
            .clamp(self.movement_range[0], self.movement_range[1]);
        self.transform2d = Transform2d {
            position: [new_x, self.transform2d.position[1]],
            scale: self.transform2d.scale,
        };
        self.dirty = true;
    }
}

impl GameObject for Bat {
    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn transform2d(&self) -> &Transform2d {
        &self.transform2d
    }

    fn renderable(&self) -> &Renderable {
        &self.renderable
    }

    fn update(&mut self, _time: f32, delta_time: f32) {
        self.dirty = false;
        if self.left_pressed && !self.right_pressed {
            self.try_move_x(-delta_time * self.speed);
        }
        else if self.right_pressed && !self.left_pressed {
            self.try_move_x(delta_time * self.speed);
        }
    }

    fn on_event(&mut self, event: Event) {
        match event {
            Event::LeftInput(state) => {
                self.left_pressed = if state == Pressed { true } else { false };
            }
            Event::RightInput(state) => {
                self.right_pressed = if state == Pressed { true } else { false };
            }
            _ => ()
        }
    }
}