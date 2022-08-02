use bevy::prelude::{Entity, IVec2};

#[derive(Clone, Copy, Debug)]
pub enum Touchee {
    Entity(Entity),
    Tile(IVec2)
}

#[derive(Clone, Copy, Debug)]
pub struct TouchEvent {
    pub toucher:Entity,
    pub touchee:Touchee
}