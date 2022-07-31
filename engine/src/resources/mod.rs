mod config;
pub use config::*;

mod tilemap;
pub use tilemap::*;

mod snapshot;
pub use snapshot::*;

mod history;
pub use history::*;

use bevy::prelude::*;
pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Config::default())
        .insert_resource(Tilemap::default())
        .insert_resource(Snapshot::default())
        .insert_resource(History::default());
    }
}