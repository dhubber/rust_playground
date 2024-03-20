mod bat;
mod wall;
mod ball;
mod brick;
mod gameover_trigger;

use rpgf::{Camera2d, Color4, Event, EventListener, EventType, Game, Scene, SceneUpdate, Update, WindowParameters};
use crate::ball::Ball;
use crate::bat::Bat;
use crate::BreakoutGameState::Playing;
use crate::brick::{Brick, BRICK_SPACING};
use crate::gameover_trigger::GameOverTrigger;
use crate::wall::{Wall, WALL_THICKNESS};

const LEVEL_WIDTH: f32 = 1.0;
const LEVEL_HEIGHT: f32 = 1.0;
const START_POSITION: [f32; 2] = [0.0, -0.45];
const NUM_BRICKS: [u32; 2] = [8, 6];
const COLOR_LERP_TIME: f32 = 1.0;
const VICTORY_BG_COLOR: Color4 = Color4 { r: 0.0, g: 0.2, b: 0.0, a: 1.0 };
const DEFEAT_BG_COLOR: Color4 = Color4 { r: 0.2, g: 0.0, b: 0.0, a: 1.0 };

#[derive(Debug)]
pub enum BreakoutGameState {
    Setup,
    Playing,
    Victory,
    Defeat
}

pub struct Breakout {
    state: BreakoutGameState,
    num_bricks: u32,
    bat_id: u128,
}

impl Game for Breakout {

    fn new() -> Self {
        Breakout {
            state: BreakoutGameState::Setup,
            num_bricks: NUM_BRICKS[0] * NUM_BRICKS[1],
            bat_id: 0,
        }
    }

    fn create_scene(&mut self) -> Scene {
        let mut scene = Scene::new("breakout".to_string());
        let gameover_trigger_id = scene.add_to_scene(Box::new(GameOverTrigger::new()));
        self.bat_id = scene.add_to_scene(Box::new(Bat::new()));
        let _ball_id = scene.add_to_scene(Box::new(Ball::new(self.bat_id, gameover_trigger_id)));
        let _left_wall_id = scene.add_to_scene(Box::new(
            Wall::new([0.5 * (WALL_THICKNESS - LEVEL_WIDTH), 0.0], [WALL_THICKNESS, LEVEL_HEIGHT])
        ));
        let _right_wall_id = scene.add_to_scene(Box::new(
            Wall::new([0.5 * (LEVEL_WIDTH - WALL_THICKNESS), 0.0], [WALL_THICKNESS, LEVEL_HEIGHT])
        ));
        let _top_wall_id = scene.add_to_scene(Box::new(
            Wall::new([0.0, 0.5 * (LEVEL_HEIGHT - WALL_THICKNESS)], [LEVEL_WIDTH - 2.0 * WALL_THICKNESS, WALL_THICKNESS])
        ));

        for j in 0..NUM_BRICKS[1] {
            let y = (0.5 * LEVEL_HEIGHT) - WALL_THICKNESS - (((j + 2) as f32) * BRICK_SPACING[1]);
            for i in 0..NUM_BRICKS[0] {
                let x = ((i as f32) - 0.5 * (NUM_BRICKS[0] as f32) + 0.5) * BRICK_SPACING[0];
                let _brick_id = scene.add_to_scene(Box::new(Brick::new([x, y])));
                println!("{x} {y}");
            }
        }
        self.change_state(Playing);
        scene
    }

    fn camera(&self) -> Camera2d {
        Camera2d {
            position: [0.0, 0.0],
            width: LEVEL_WIDTH,
            height: LEVEL_HEIGHT,
            z_near: -1.0,
            z_far: 1.0
        }
    }

    fn player_id(&self) -> u128 {
        self.bat_id
    }
}

impl Update for Breakout {
    fn update(&mut self, time: f32, delta_time: f32) -> Option<Vec<Event>> {
        None
    }
}

impl EventListener for Breakout {
    fn on_event(&mut self, event: &Event) -> Option<Vec<Event>> {
        match event.event_type {
            EventType::BrickDestroyed => {
                self.num_bricks -= 1;
                if self.num_bricks == 0 {
                    self.change_state(BreakoutGameState::Victory);
                    return Some(vec![Event { id: 0, event_type: EventType::PlayerWins },
                                     Event { id: 0, event_type: EventType::BackgroundColor(VICTORY_BG_COLOR) }])
                }
                println!("Num bricks: {}    State: {:?}", self.num_bricks, self.state);
            }
            EventType::BallOutOfBounds => {
                self.change_state(BreakoutGameState::Defeat);
                return Some(vec![Event { id: 0, event_type: EventType::PlayerLoses },
                                 Event { id: 0, event_type: EventType::BackgroundColor(DEFEAT_BG_COLOR) }])
            }
            _ => ()
        }
        None
    }
}

impl Breakout {
    fn change_state(&mut self, new_state: BreakoutGameState) {
        self.state = new_state;
        println!("New state: {:?}", self.state);
    }
}


fn main() {
    rpgf::Application::new()
        .set_window_parameters(WindowParameters {
            width: 800,
            height: 800,
            background_color: Color4 { r: 0.0, g: 0.0, b: 0.025, a: 0.0 },
            title: String::from("Hello World"), })
        .run::<Breakout>();
}
