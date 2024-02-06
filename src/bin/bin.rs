use rpgf::{Camera2d, Color4, GameObject, Scene, WindowParameters};

fn main() {
    let window_parameters = WindowParameters {
        width: 800,
        height: 800,
        background_color: Color4{ r: 0.0, g: 0.0, b: 0.025, a: 0.0 },
        title: String::from("Hello World"),
    };

    let camera2d = Camera2d {
        position: [0.0, 0.0],
        width: 1.0,
        height: 1.0,
        z_near: -1.0,
        z_far: 1.0
    };

    let mut scene = Scene::new("breakout".to_string());
    scene.new_object([0.0, -0.4], [0.2, 0.05], [0.9, 0.9, 0.6, 1.0]);

    rpgf::run(window_parameters, camera2d, scene);
}
