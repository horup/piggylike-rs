use core::bevy as bevy;
use bevy::prelude::*;

mod shape;
pub use shape::*;



pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, _app: &mut App) {
    }
}