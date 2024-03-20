use rpgf::{GameObject, Renderable, Transform2d};
use crate::LEVEL_HEIGHT;

pub struct GameOverTrigger {
    pub transform2d: Transform2d,
    pub renderable: Renderable,
}

impl GameOverTrigger {
    pub fn new() -> Self {
        GameOverTrigger {
            transform2d: Transform2d {
                position: [0.0, -0.6*LEVEL_HEIGHT],
                scale: [1.0, 0.01]
            },
            renderable: Renderable {
                color: [0.0, 0.0, 0.0, 0.0]
            },
        }
    }
}

impl GameObject for GameOverTrigger {

    fn transform2d(&self) -> &Transform2d {
        &self.transform2d
    }

    fn renderable(&self) -> &Renderable {
        &self.renderable
    }
}