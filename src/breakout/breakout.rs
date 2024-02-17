mod bat;
mod wall;
mod ball;
mod brick;

use rpgf::{Camera2d, Color4, Event, Game, Scene, WindowParameters};
use crate::ball::Ball;
use crate::bat::Bat;
use crate::brick::{Brick, BRICK_SPACING};
use crate::wall::{Wall, WALL_THICKNESS};

const LEVEL_WIDTH: f32 = 1.0;
const LEVEL_HEIGHT: f32 = 1.0;
const START_POSITION: [f32; 2] = [0.0, -0.45];
const NUM_BRICKS: [u32; 2] = [8, 6];

pub enum BreakoutGameState {
    Setup,
    Playing,
    Victory,
    Defeat
}

pub struct Breakout {
    scene: Scene,
    state: BreakoutGameState,
    bat_id: u128,
}

impl Game for Breakout {

    fn setup(&mut self) {
        self.scene = Scene::new("breakout".to_string());
        self.bat_id = self.scene.add_to_scene(Box::new(Bat::new()));

        let _ball_id = self.scene.add_to_scene(Box::new(Ball::new(self.bat_id)));

        let _left_wall_id = self.scene.add_to_scene(Box::new(
            Wall::new([0.5 * (WALL_THICKNESS - LEVEL_WIDTH), 0.0], [WALL_THICKNESS, LEVEL_HEIGHT])
        ));
        let _right_wall_id = self.scene.add_to_scene(Box::new(
            Wall::new([0.5 * (LEVEL_WIDTH - WALL_THICKNESS), 0.0], [WALL_THICKNESS, LEVEL_HEIGHT])
        ));
        let _top_wall_id = self.scene.add_to_scene(Box::new(
            Wall::new([0.0, 0.5 * (LEVEL_HEIGHT - WALL_THICKNESS)], [LEVEL_WIDTH - 2.0 * WALL_THICKNESS, WALL_THICKNESS])
        ));

        for j in 0..NUM_BRICKS[1] {
            let y = (0.5 * LEVEL_HEIGHT) - WALL_THICKNESS - (((j + 2) as f32) * BRICK_SPACING[1]);
            for i in 0..NUM_BRICKS[0] {
                let x = ((i as f32) - 0.5 * (NUM_BRICKS[0] as f32) + 0.5) * BRICK_SPACING[0];
                let _brick_id = self.scene.add_to_scene(Box::new(Brick::new([x, y])));
                println!("{x} {y}");
            }
        }
    }

    fn scene(&self) -> &Scene {
        &self.scene
    }

    fn update(&mut self, time: f32, delta_time: f32) {
        self.scene.update(time, delta_time);
    }

    fn on_event(&mut self, id: u128, event: Event) {
        self.scene.on_event(id, event);
    }

    fn player_id(&self) -> u128 {
        self.bat_id
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
        width: LEVEL_WIDTH,
        height: LEVEL_HEIGHT,
        z_near: -1.0,
        z_far: 1.0
    };

    let mut game = Box::new(Breakout{
        scene: Scene::new("breakout".to_string()),
        state: BreakoutGameState::Setup,
        bat_id: 0,
    });
    game.setup();

    rpgf::run(window_parameters, camera2d, game);
}
