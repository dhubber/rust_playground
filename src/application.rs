use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use glium::buffer::BufferMode::Default;
use crate::{AudioSystem, Camera2d, Color4, Event, EventType, Game, SceneUpdate, Update, WindowParameters};
use crate::collision::CollisionSolver;
use crate::event::EventManager;
use crate::renderer::Renderer;

pub struct Application {
    window_parameters: WindowParameters,
}

impl Application {

    pub fn new() -> Self {
        Application {
            window_parameters: WindowParameters {
                width: 800,
                height: 800,
                background_color: Color4 { r: 0.0, g: 0.0, b: 0.025, a: 0.0 },
                title: String::from("Application")
            }
        }
    }

    pub fn set_window_parameters(&mut self, window_parameters: WindowParameters) -> &mut Self {
        self.window_parameters = window_parameters;
        self
    }

    pub fn run<T: Game + 'static>(&mut self)
    {
        let game = Rc::new(RefCell::new(T::new()));
        let main_scene = Rc::new(RefCell::new(game.borrow_mut().create_scene()));
        let player_id = game.borrow().player_id();
        let camera2d = game.borrow().camera();

        let event_loop = winit::event_loop::EventLoopBuilder::new().build();
        let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_inner_size(self.window_parameters.width, self.window_parameters.height)
            .build(&event_loop);

        let mut renderer = Rc::new(RefCell::new(Renderer::new(display, &camera2d, &self.window_parameters.background_color)));
        let mut collision_solver = Rc::new(RefCell::new(CollisionSolver::new()));
        let mut audio_system = Rc::new(RefCell::new(AudioSystem::new()));
        audio_system.borrow_mut().register_audio_file("ball_collision".to_string(), Path::new("ball_collision.mp3"));

        let mut event_manager = EventManager::new();
        event_manager.register_listener(game.clone());
        event_manager.register_listener(main_scene.clone());
        event_manager.register_listener(renderer.clone());
        event_manager.register_listener(collision_solver.clone());
        event_manager.register_listener(audio_system.clone());

        let start = Instant::now();
        let mut last_time: f32 = 0.0;
        let mut time: f32 = 0.0;
        let mut delta: f32 = 0.0;
        let mut num_frames = 0;

        event_loop.run(move |event, _ , control_flow| {

            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                    winit::event::WindowEvent::Resized(window_size) => {
                        renderer.borrow_mut().resize(window_size.into());
                    },
                    winit::event::WindowEvent::KeyboardInput { device_id: _device_id, input, is_synthetic: _is_synthetic } => match input.virtual_keycode {
                        None => (),
                        Some(key_code) => match key_code {
                            winit::event::VirtualKeyCode::Left => {
                                event_manager.broadcast_event_queue(Some(vec![
                                    Event{ id: player_id, event_type: EventType::LeftInput(input.state) }
                                ]));
                            },
                            winit::event::VirtualKeyCode::Right => {
                                event_manager.broadcast_event_queue(Some(vec![
                                    Event{ id: player_id, event_type: EventType::RightInput(input.state) }
                                ]));
                            },
                            winit::event::VirtualKeyCode::Space => {
                                event_manager.broadcast_event_queue(Some(vec![
                                    Event{ id: player_id, event_type: EventType::FireInput(input.state) }
                                ]));
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
                    time = start.elapsed().as_secs_f32();
                    delta = time - last_time;
                    last_time = time;
                    num_frames += 1;
                    let fps: f32 = (num_frames as f32) / time;
                    if num_frames % 100 == 0 {
                        println!("FPS: {fps} {time}");
                        //game.console_log();
                    };
                    main_scene.borrow_mut().update(time, delta);
                    game.borrow_mut().update(time, delta);
                    let events = collision_solver.borrow_mut().update(&main_scene.borrow(), time, delta);
                    event_manager.broadcast_event_queue(events);
                    _window.request_redraw();
                }
                winit::event::Event::RedrawRequested(_) => {
                    renderer.borrow_mut().update(&main_scene.borrow(), time, delta);
                }
                _ => (),
            };
        });
    }
}