use rpgf::{Event, GameObject, Renderable, Transform2d};
use crate::bat::BAT_SIZE;
use crate::START_POSITION;

pub const BALL_SIZE: f32 = 0.0125;
pub const BALL_COLOR: [f32; 4] = [0.8, 0.2, 0.4, 1.0];
pub const INIT_BALL_SPEED: f32 = 0.5;
pub const BALL_ACCEL: f32 = 0.01;
pub const INIT_DIRECTION: [f32; 2] = [0.707, 0.707];

pub struct Ball {
    pub transform2d: Transform2d,
    pub renderable: Renderable,
    direction: [f32; 2],
    speed: f32,
    bat_id: u128
}

impl Ball {
    pub fn new(bat_id: u128) -> Self {
        Ball {
            transform2d: Transform2d {
                position: [START_POSITION[0], START_POSITION[1] + 0.5*(BAT_SIZE[1] + BALL_SIZE)],
                scale: [BALL_SIZE, BALL_SIZE],
            },
            renderable: Renderable {
                color: BALL_COLOR,
            },
            direction: INIT_DIRECTION,
            speed: INIT_BALL_SPEED,
            bat_id
        }
    }
}

impl GameObject for Ball {
    fn transform2d(&self) -> &Transform2d {
        &self.transform2d
    }

    fn renderable(&self) -> &Renderable {
        &self.renderable
    }

    fn update(&mut self, _time: f32, delta_time: f32) {
        let dist = self.speed * delta_time;
        let new_position = [self.transform2d.position[0] + dist * self.direction[0],
            self.transform2d.position[1] + dist * self.direction[1]];
        self.transform2d = Transform2d {
            position: new_position,
            scale: self.transform2d.scale
        };
    }

    fn on_event(&mut self, event: Event) {
        match event {
            Event::OnCollisionEnter{other, aab1, aab2, .. } => {
                if other == self.bat_id {
                    if self.direction[1] < 0.0 {
                        self.speed += BALL_ACCEL;
                        let bat_size = aab2.max[0] - aab2.min[0];
                        let bat_x_pos = 0.5*(aab2.min[0] + aab2.max[0]);
                        let diff = self.transform2d.position[0] - bat_x_pos;
                        let angle = 1.5 * 0.5 * 3.14157 * diff / bat_size;
                        self.direction = [angle.sin(), angle.cos()];
                    }
                }
                else {
                    let brick_pos = aab2.centre();
                    let diff = [brick_pos[0] - self.transform2d.position[0], brick_pos[1] - self.transform2d.position[1]];
                    let overlap = [(aab2.max[0] - aab1.min[0]).min(aab1.max[0] - aab2.min[0]),
                        (aab2.max[1] - aab1.min[1]).min(aab1.max[1] - aab2.min[1])];
                    if overlap[1].abs() > overlap[0].abs() && self.direction[0]*diff[0] > 0.0 {
                        self.direction[0] = - self.direction[0];
                    }
                    if overlap[0].abs() > overlap[1].abs() && self.direction[1]*diff[1] > 0.0 {
                        self.direction[1] = - self.direction[1];
                    }
                }
            }
            _ => ()
        }
    }
}