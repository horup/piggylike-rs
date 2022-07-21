use macroquad::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub pos:Vec2,
    pub visible_tiles:f32
}

impl Default for Camera {
    fn default() -> Self {
        Self { pos: Default::default(), visible_tiles: 24.0 }
    }
}
