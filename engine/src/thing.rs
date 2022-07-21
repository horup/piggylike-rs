use generational_arena::Index;
use macroquad::prelude::Vec2;

#[derive(Clone, Copy)]
pub struct Thing {
    pub id: Index,
    pub atlas: u32,
    pub atlas_index: u16,
    pub pos: Vec2,
}

impl Default for Thing {
    fn default() -> Self {
        Self {
            id: Index::from_raw_parts(0, 0),
            atlas: Default::default(),
            atlas_index: Default::default(),
            pos: Default::default(),
        }
    }
}
