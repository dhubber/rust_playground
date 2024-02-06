use glam::Mat4;
use glium::glutin::surface::WindowSurface;
use glium::{Program, Surface, VertexBuffer};
use crate::{Camera2d, Color4, Scene, Vertex2d};

pub struct Renderer {
    display: glium::Display<WindowSurface>,
    vertex_buffer: VertexBuffer<Vertex2d>,
    indices: glium::index::NoIndices,
    program: Program,
    view: Mat4,
    projection: Mat4,
}

impl Renderer {
    pub fn new(display: glium::Display<WindowSurface>, camera2d: Camera2d) -> Self {
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
    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;
    void main() {
        gl_Position = projection * view * model * vec4(position, 0.0, 1.0);
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
        // OpenGL buffer and shader for rectangles
        let vertex_buffer = glium::VertexBuffer::new(&display, &rectangle_vertex_data).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let view = camera2d.view_matrix();
        let projection = camera2d.projection_matrix();

        Renderer {
            display,
            vertex_buffer,
            indices,
            program,
            view,
            projection
        }
    }

    pub fn render(&self, scene: &Scene, color: &Color4, x_bat: f32) {
        let mut frame = self.display.draw();
        frame.clear_color(color.r, color.g, color.b, color.a);

        for (id,object) in scene.objects.clone().into_iter() {
            let model_matrix = Mat4::from_cols_array_2d(
                &[
                    [object.scale[0], 0.0, 0.0, 0.0],
                    [0.0, object.scale[1], 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [object.position[0] + x_bat , object.position[1], 0.0, 1.0f32],
                ],
            );
            let uniforms = uniform! {
                        position: object.position,
                        model: model_matrix.to_cols_array_2d(),
                        view: self.view.to_cols_array_2d(),
                        projection: self.projection.to_cols_array_2d(),
                        col: object.color,
                    };
            frame.draw(&self.vertex_buffer, &self.indices, &self.program, &uniforms,
                       &Default::default()).unwrap();
        }
        frame.finish().unwrap();
    }

    pub fn resize(&self, new_size:(u32, u32)) {
        self.display.resize(new_size);
    }
}