use rpgf::{Event, EventType, GameObject, Renderable, Transform2d};

pub const BRICK_SIZE: [f32; 2] = [0.095, 0.06];
pub const BRICK_SPACING: [f32; 2] = [0.1, 0.065];
pub const BRICK_COLOR_1: [f32; 4] = [0.1, 0.5, 0.15, 1.0];
pub const BRICK_COLOR_2: [f32; 4] = [0.5, 0.9, 0.6, 1.0];

pub struct Brick {
    pub transform2d: Transform2d,
    pub renderable: Renderable,
    pub num_hits: u32,
}

impl Brick {
    pub fn new(position: [f32; 2]) -> Self {
        Brick {
            transform2d: Transform2d {
                position,
                scale: BRICK_SIZE,
            },
            renderable: Renderable {
                color: BRICK_COLOR_1,
            },
            num_hits: 0
        }
    }
}

impl GameObject for Brick {
    fn is_active(&self) -> bool {
        self.num_hits < 2
    }
    fn transform2d(&self) -> &Transform2d {
        &self.transform2d
    }

    fn renderable(&self) -> &Renderable {
        &self.renderable
    }

    fn on_event(&mut self, event: &Event) -> Option<Vec<Event>> {
        match event.event_type {
            EventType::OnCollisionEnter{..} => {
                self.num_hits += 1;
                if self.num_hits == 1 {
                    self.renderable.color = BRICK_COLOR_2
                }
                if self.num_hits == 2 {
                    return Some(vec![Event { id: 0, event_type: EventType::BrickDestroyed}])
                }
            }
            _ => ()
        }
        None
    }
}