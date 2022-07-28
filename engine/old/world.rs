use generational_arena::{Arena, Index};
use parry2d::na::Dynamic;
use serde::{Serialize, Deserialize};

use crate::{Tilemap, Thing, Camera, script};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct World {
    pub iterations:i64,
    pub tilemap:Tilemap,
    pub things:Arena<Thing>,
    pub camera:Camera
}

impl World {
    pub fn get_iterations(&mut self) -> i64 {
        self.iterations
    }
    pub fn set_iterations(&mut self, value:i64) {
        self.iterations = value;
    }

    pub fn get_things(&mut self) -> Arena<Thing> {
        return self.things.clone();
    }
}