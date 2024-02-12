use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use crate::aab::AxisAlignedBox;
use crate::Event;
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

    pub fn solve(&mut self, scene: &mut Scene) {
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
                    if AxisAlignedBox::overlap(&aab1, &aab2) {
                        self.new_collision_set.insert(Collision{ obj1: id1.clone(), obj2: id2.clone(), aab1, aab2 });
                    }
                }
            }
        }

        let diff1 = self.old_collision_set.difference(&self.new_collision_set);
        for a in diff1 {
            scene.on_event(a.obj1, Event::OnCollisionExit { id: a.obj1, other: a.obj2, aab1: a.aab1, aab2: a.aab2 });
        }

        let diff2 = self.new_collision_set.difference(&self.old_collision_set);
        for a in diff2 {
            scene.on_event(a.obj1, Event::OnCollisionEnter { id: a.obj1, other: a.obj2, aab1: a.aab1, aab2: a.aab2 });
        }
    }
}