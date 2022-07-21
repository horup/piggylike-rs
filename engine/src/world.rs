use generational_arena::Arena;

use crate::{Tilemap, Thing};

#[derive(Default, Clone)]
pub struct World {
    pub tilemap:Tilemap,
    pub things:Arena<Thing>
}
