mod camera;
mod game_object;
mod scene;
mod renderer;
mod event;
mod collision;
mod aab;
mod game;
mod audio;
mod application;

#[macro_use]
extern crate glium;
extern crate glam;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use glium::{implement_vertex};
use std::time::Instant;
pub use camera::Camera2d;
pub use game_object::*;
pub use scene::*;
pub use event::*;
use crate::collision::CollisionSolver;
use crate::event::EventManager;
pub use crate::application::*;
pub use crate::audio::*;
pub use crate::game::Game;
pub use crate::game::*;
use crate::renderer::Renderer;

#[derive(Copy, Clone, Debug)]
pub struct Color4 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug)]
pub struct WindowParameters {
    pub width: u32,
    pub height: u32,
    pub background_color: Color4,
    pub title: String
}

/// Simple 2d vertex
#[derive(Copy, Clone)]
pub struct Vertex2d {
    pub position: [f32; 2],
}
implement_vertex!(Vertex2d, position);