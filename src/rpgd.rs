#[macro_use]
extern crate glium;

use glium::{implement_vertex, Surface};

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

#[derive(Copy, Clone, Debug)]
pub struct GameObject {
    pub position: [f32; 2],
    pub scale: [f32; 2],
    pub color: [f32; 4]
}


/// Creates a window using the glium crate
pub fn run(window_parameters: &WindowParameters, objects: Vec<GameObject>) {

    let rectangle_vertex_data = vec![
        Vertex2d{ position: [-0.5, -0.5]},
        Vertex2d{ position: [0.5, -0.5]},
        Vertex2d{ position: [0.5, 0.5]},
        Vertex2d{ position: [-0.5, -0.5]},
        Vertex2d{ position: [0.5, 0.5]},
        Vertex2d{ position: [-0.5, 0.5]},
    ];

    let vertex_shader_src = r#"
    #version 140
    in vec2 position;
    uniform mat4 matrix;
    void main() {
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

    let fragment_shader_src = r#"
    #version 140
    out vec4 color;
    uniform vec4 col;
    void main() {
        color = col;
    }
"#;

    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let vertex_buffer = glium::VertexBuffer::new(&display, &rectangle_vertex_data).unwrap();
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
                let _x = t.sin() * 0.5;
                let mut frame = display.draw();
                frame.clear_color(color.r, color.g, color.b, color.a);
                for object in objects.clone().into_iter() {
                    let uniforms = uniform! {
                        position: object.position,
                        matrix: [
                            [object.scale[0], 0.0, 0.0, 0.0],
                            [0.0, object.scale[1], 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [object.position[0], object.position[1], 0.0, 1.0f32],
                        ],
                        col: object.color,
                    };
                    frame.draw(&vertex_buffer, &indices, &program, &uniforms,
                               &Default::default()).unwrap();
                }
                frame.finish().unwrap();
            }
            _ => (),
        };
    });
}
