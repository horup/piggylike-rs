mod systems;
mod api;
pub use systems::*;
use bevy::prelude::*;

pub struct ScriptPlugin;

impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}