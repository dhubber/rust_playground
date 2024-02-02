use std::env::args;
use glium::program::ShaderType::Vertex;
use rpgf::{Color4, Vertex2d, WindowParameters};

fn main() {
    let window_parameters = WindowParameters {
        background_color: Color4{ r: 0.0, g: 0.0, b: 0.1, a: 0.0 },
        title: String::from("Hello World"),
    };
    let vertex1 = Vertex2d{ position: [-0.5, -0.5]};
    let vertex2 = Vertex2d{ position: [0.0, 0.5]};
    let vertex3 = Vertex2d{ position: [0.5, -0.25]};
    let shape = vec![vertex1, vertex2, vertex3];
    rpgf::run(&window_parameters, shape);
}
