use generational_arena::Arena;
use serde::{Serialize, Deserialize};

use crate::{Tilemap, Thing, Camera};

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

    pub fn register(script_engine:&mut rhai::Engine) {
        script_engine.register_type_with_name::<Self>("World");
        script_engine.register_get_set("iterations", Self::get_iterations, Self::set_iterations);
    }
}