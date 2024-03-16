use std::cell::RefCell;
use std::rc::Rc;
use winit::event::ElementState;
use crate::aab::AxisAlignedBox;

#[derive(Clone,Debug)]
pub enum EventType {
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
    PlayAudio(String),
}

#[derive(Clone,Debug)]
pub struct Event {
    pub id: u128,
    pub event_type: EventType
}

pub trait EventListener {
    fn on_event(&mut self, event: &Event) -> Option<Vec<Event>>;
}

pub struct EventManager {
    listeners: Vec<Rc<RefCell<dyn EventListener>>>,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            listeners: Vec::new(),
        }
    }

    pub fn register_listener(&mut self, listener: Rc<RefCell<dyn EventListener>>) {
        self.listeners.push(listener);
    }

    pub fn broadcast_event_queue(&self, optional_events: Option<Vec<Event>>) {
        match optional_events {
            Some(events) => {
                let mut queue = events.clone();
                while queue.len() > 0 {
                    let mut new_events: Vec<Event> = Vec::new();
                    for event in queue.iter() {
                        //println!("Broadcasting event {:?} to {} listeners", event, self.listeners.len());
                        for listener in self.listeners.iter() {
                            if let Some(optional_events) = listener.borrow_mut().on_event(event) {
                                new_events.extend(optional_events);
                            }
                        }
                    }
                    queue = new_events.clone();
                }
            }
            None => ()
        }
    }
}