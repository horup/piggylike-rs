use core::bevy as bevy;
use bevy::prelude::*;
use bevy::ecs as bevy_ecs;

mod shape;
pub use shape::*;



pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
    }
}