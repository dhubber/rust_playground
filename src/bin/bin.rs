use rpgf::{Color4, GameObject, WindowParameters};

fn main() {
    let window_parameters = WindowParameters {
        background_color: Color4{ r: 0.0, g: 0.0, b: 0.025, a: 0.0 },
        title: String::from("Hello World"),
    };
    let objects: Vec<GameObject> = vec![
        GameObject{
            position: [0.4, -0.5],
            scale: [0.1, 0.5],
            color: [0.0, 0.8, 0.5, 1.0],
        },
        GameObject{
            position: [-0.3, 0.5],
            scale: [0.85, 0.2],
            color: [0.9, 0.0, 0.0, 1.0],
        },
    ];
    rpgf::run(&window_parameters, objects);
}
