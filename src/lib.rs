#[macro_use]
extern crate glium;
use glium::{implement_vertex, Surface};
//use glium::buffer::BufferMode::Default;
use winit::event_loop;

#[derive(Copy, Clone, Debug)]
pub struct Color4 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug)]
pub struct WindowParameters {
    pub background_color: Color4,
    pub title: String
}

/// Simple 2d vertex
#[derive(Copy, Clone)]
pub struct Vertex2d {
    pub position: [f32; 2],
}
implement_vertex!(Vertex2d, position);

/// Creates a window using the glium crate
pub fn run(window_parameters: &WindowParameters, shape: Vec<Vertex2d>) {

    let vertex_shader_src = r#"
    #version 140
    in vec2 position;
    uniform float x;
    void main() {
        vec2 pos = position;
        pos.x += x;
        gl_Position = vec4(pos, 0.0, 1.0);
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
    let color = window_parameters.background_color.clone();
    frame.clear_color(color.r, color.g, color.b, color.a);
    frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
               &Default::default()).unwrap();
    frame.finish().unwrap();

    let mut t: f32 = 0.0;

    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                }
                _ => (),
            },
            winit::event::Event::RedrawEventsCleared => {
                _window.request_redraw();
            },
            winit::event::Event::RedrawRequested(_) => {
                t += 0.02;
                let x = t.sin() * 0.5;
                let mut frame = display.draw();
                frame.clear_color(color.r, color.g, color.b, color.a);
                frame.draw(&vertex_buffer, &indices, &program, &uniform! {x: x},
                           &Default::default()).unwrap();
                frame.finish().unwrap();
            }
            _ => (),
        };
    });
}
