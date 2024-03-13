use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use crate::aab::AxisAlignedBox;
use crate::{Event, EventType};
use crate::event::EventListener;
use crate::game::{Game, SceneUpdate};
pub use crate::Scene;

#[derive(Copy, Clone, Debug)]
pub struct Collision {
    pub obj1: u128,
    pub obj2: u128,
    pub aab1: AxisAlignedBox,
    pub aab2: AxisAlignedBox,
}

impl PartialEq<Self> for Collision {
    fn eq(&self, other: &Self) -> bool {
        self.obj1.eq(&other.obj1) && self.obj2.eq(&other.obj2)
    }
}

impl Eq for Collision {}

impl Hash for Collision {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.obj1.hash(state);
        self.obj2.hash(state);
    }
}

pub struct CollisionSolver {
    pub old_collision_set: HashSet<Collision>,
    pub new_collision_set: HashSet<Collision>
}

impl CollisionSolver {
    pub fn new() -> CollisionSolver {
        CollisionSolver {
            old_collision_set: HashSet::new(),
            new_collision_set: HashSet::new(),
        }
    }
}

impl SceneUpdate for CollisionSolver {
    fn update(&mut self, scene: &Scene, _time: f32, _delta_time: f32) -> Option<Vec<Event>> {
        self.old_collision_set = self.new_collision_set.clone();
        self.new_collision_set.clear();
        for (id1, obj1) in scene.objects.iter() {
            if !obj1.is_active() {
                continue;
            }
            let aab1 = obj1.aa_box();
            for (id2, obj2) in scene.objects.iter() {
                if !obj2.is_active() {
                    continue;
                }
                if id1 != id2 {
                    let aab2 = obj2.aa_box();
                    if AxisAlignedBox::is_overlapping(&aab1, &aab2) {
                        self.new_collision_set.insert(Collision{ obj1: id1.clone(), obj2: id2.clone(), aab1, aab2 });
                    }
                }
            }
        }

        // Create list of collision events to be broadcast to all listeners
        let mut event_queue: Vec<Event> = Vec::new();

        let diff1 = self.old_collision_set.difference(&self.new_collision_set);
        for a in diff1 {
            event_queue.push(Event {
                id: a.obj1,
                event_type: EventType::OnCollisionExit { id: a.obj1, other: a.obj2, aab1: a.aab1, aab2: a.aab2 }
            });
        }

        let diff2 = self.new_collision_set.difference(&self.old_collision_set);
        for a in diff2 {
            event_queue.push(Event {
                id: a.obj1,
                event_type: EventType::OnCollisionEnter { id: a.obj1, other: a.obj2, aab1: a.aab1, aab2: a.aab2 }
            });
        }
        Some(event_queue)
    }
}

impl EventListener for CollisionSolver {
    fn on_event(&mut self, event: &Event) -> Option<Vec<Event>> {
        None
    }
}