#[macro_use]
extern crate glium;
use glium::{implement_vertex, Surface};
//use glium::buffer::BufferMode::Default;
use winit::event_loop;

pub struct Color4 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

/// Simple 2d vertex
#[derive(Copy, Clone)]
pub struct Vertex2d {
    pub position: [f32; 2],
}
implement_vertex!(Vertex2d, position);

/// Creates a window using the glium crate
pub fn run(bgColor: Color4, shape: Vec<Vertex2d>) {

    let vertex_shader_src = r#"
    #version 140
    in vec2 position;
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

    let fragment_shader_src = r#"
    #version 140
    out vec4 color;
    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut frame = display.draw();
    frame.clear_color(bgColor.r, bgColor.g, bgColor.b, bgColor.a);
    frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
               &Default::default()).unwrap();
    frame.finish().unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                _ => (),
            },
            _ => (),
        };
    });
}
