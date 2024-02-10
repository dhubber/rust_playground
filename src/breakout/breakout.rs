mod bat;

use std::rc::Rc;
use winit::event::ElementState::Pressed;
use rpgf::{Camera2d, Color4, Event, GameObject, Renderable, Scene, Transform2d, WindowParameters};
use crate::bat::Bat;

const LEVEL_WIDTH: f32 = 1.0;
const LEVEL_HEIGHT: f32 = 1.0;
const WALL_THICKNESS: f32 = 0.05;
const START_POSITION: [f32; 2] = [0.0, -0.45];


fn main() {
    let window_parameters = WindowParameters {
        width: 800,
        height: 800,
        background_color: Color4 { r: 0.0, g: 0.0, b: 0.025, a: 0.0 },
        title: String::from("Hello World"),
    };

    let camera2d = Camera2d {
        position: [0.0, 0.0],
        width: LEVEL_WIDTH,
        height: LEVEL_HEIGHT,
        z_near: -1.0,
        z_far: 1.0
    };

    let mut scene = Scene::new("breakout".to_string());
    let bat_id = scene.add_to_scene(Box::new(Bat::new()));

    rpgf::run(window_parameters, camera2d, scene, bat_id);
}
