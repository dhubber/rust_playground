use winit::event::ElementState;

pub enum Event {
    Quit,
    LeftInput(ElementState),
    RightInput(ElementState),
    FireInput(ElementState),
    OnCollisionEnter {id: u128, other: u128},
    OnCollisionExit {id: u128, other: u128},
}