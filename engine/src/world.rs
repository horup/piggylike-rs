use nanoserde::{DeJson, SerJson};
use generational_arena::Arena;

use crate::{Tilemap, Thing, Camera};

#[derive(Default, Clone)]
pub struct World {
    pub iterations:u64,
    pub tilemap:Tilemap,
    pub things:Arena<Thing>,
    pub camera:Camera
}
