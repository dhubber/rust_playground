use winit::event::ElementState;
use crate::aab::AxisAlignedBox;

#[derive(Debug)]
pub enum Event {
    Quit,
    PlayerWins,
    PlayerLoses,
    BrickDestroyed,
    BallOutOfBounds,
    LeftInput(ElementState),
    RightInput(ElementState),
    FireInput(ElementState),
    OnCollisionEnter {id: u128, other: u128, aab1: AxisAlignedBox, aab2: AxisAlignedBox},
    OnCollisionExit {id: u128, other: u128, aab1: AxisAlignedBox, aab2: AxisAlignedBox},
    MoveToPosition {id: u128, position: [f32; 2]},
    PlayAudio,
}