mod config;
pub use config::*;

mod tilemap;
pub use tilemap::*;

use bevy::prelude::*;
pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Config::default())
        .insert_resource(Tilemap::default());
    }
}