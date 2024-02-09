mod camera;
mod game_object;
mod scene;
mod renderer;
mod event;

#[macro_use]
extern crate glium;
extern crate glam;

use std::rc::Rc;
use glium::{implement_vertex, Surface};
use std::time::Instant;
pub use camera::Camera2d;
pub use game_object::*;
pub use scene::*;
pub use event::{Event};
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


/// Creates a window using the glium crate
pub fn run(window_parameters: WindowParameters, camera2d: Camera2d, mut scene: Scene, player: u128)
{
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(window_parameters.width, window_parameters.height)
        .build(&event_loop);

    let renderer = Renderer::new(display, &camera2d, &window_parameters.background_color);

    let start = Instant::now();
    let mut last_time: f32 = 0.0;
    let mut num_frames = 0;

    event_loop.run(move |event, _ , control_flow| {

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                winit::event::WindowEvent::Resized(window_size) => {
                    renderer.resize(window_size.into());
                },
                winit::event::WindowEvent::KeyboardInput { device_id: _device_id, input, is_synthetic: _is_synthetic } => match input.virtual_keycode {
                    None => (),
                    Some(key_code) => match key_code {
                        winit::event::VirtualKeyCode::Left => {
                            scene.on_event(player, Event::LeftInput(input.state));
                        },
                        winit::event::VirtualKeyCode::Right => {
                            scene.on_event(player, Event::RightInput(input.state));
                        },
                        winit::event::VirtualKeyCode::Space => {
                            scene.on_event(player, Event::FireInput(input.state));
                        }
                        winit::event::VirtualKeyCode::Escape => {
                            control_flow.set_exit()
                        },
                        _ => (),
                    },
                }
                _ => (),
            },
            winit::event::Event::MainEventsCleared => {
                let time = start.elapsed().as_secs_f32();
                let delta = time - last_time;
                last_time = time;
                num_frames += 1;
                let fps: f32 = (num_frames as f32) / time;
                if num_frames % 100 == 0 { println!("FPS: {fps} {time}")};
                scene.update(time, delta);
                _window.request_redraw();
            }
            winit::event::Event::RedrawRequested(_) => {
                renderer.render(&scene);
            }
            _ => (),
        };
    });
}