use std::rc::Rc;
use winit::event::ElementState::Pressed;
use rpgf::{Camera2d, Color4, Event, GameObject, Renderable, Scene, Transform2d, WindowParameters};

const START_POSITION: [f32; 2] = [0.0, -0.4];
const BAT_SPEED: f32 = 1.0;
const BAT_SIZE: [f32; 2] = [0.2, 0.05];
const BAT_COLOR: [f32; 4] = [0.8, 0.75, 0.9, 1.0];

pub struct Bat {
    pub transform2d: Transform2d,
    pub renderable: Renderable,
    speed: f32,
    left_pressed: bool,
    right_pressed: bool,
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
            speed: BAT_SPEED,
            left_pressed: false,
            right_pressed: false,
        }
    }

    pub fn try_move_x(&mut self, delta_x: f32) {
        self.transform2d = Transform2d {
            position: [self.transform2d.position[0] + delta_x, self.transform2d.position[1]],
            scale: self.transform2d.scale
        }
    }
}

impl GameObject for Bat {
    fn transform2d(&self) -> Transform2d {
        self.transform2d
    }

    fn renderable(&self) -> Renderable {
        self.renderable
    }

    fn update(&mut self, time: f32, delta_time: f32) {
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
            Event::FireInput(state) => {
                dbg!("Bat fires");
            }
            _ => ()
        }
    }
}

fn main() {
    let window_parameters = WindowParameters {
        width: 800,
        height: 800,
        background_color: Color4 { r: 0.0, g: 0.0, b: 0.025, a: 0.0 },
        title: String::from("Hello World"),
    };

    let camera2d = Camera2d {
        position: [0.0, 0.0],
        width: 1.0,
        height: 1.0,
        z_near: -1.0,
        z_far: 1.0
    };

    let mut scene = Scene::new("breakout".to_string());
    let bat_id = scene.add_to_scene(Box::new(Bat::new()));

    rpgf::run(window_parameters, camera2d, scene, bat_id);
}
