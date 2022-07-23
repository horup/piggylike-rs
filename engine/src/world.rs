use generational_arena::Arena;
use serde::{Serialize, Deserialize};

use crate::{Tilemap, Thing, Camera};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct World {
    pub iterations:u64,
    pub tilemap:Tilemap,
    pub things:Arena<Thing>,
    pub camera:Camera
}
