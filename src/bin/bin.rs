use std::env::args;
use glium::program::ShaderType::Vertex;
use rpgf::{Color4, Vertex2d};

fn main() {
    let bgColor = Color4{ r: 0.0, g: 1.0, b: 0.0, a: 0.0 };
    let vertex1 = Vertex2d{ position: [-0.5, -0.5]};
    let vertex2 = Vertex2d{ position: [0.0, 0.5]};
    let vertex3 = Vertex2d{ position: [0.5, -0.25]};
    let shape = vec![vertex1, vertex2, vertex3];
    rpgf::run(bgColor, shape);
}
