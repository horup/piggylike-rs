use bevy::prelude::*;

pub struct EditorPlugin;

pub fn setup(mut commands:Commands) {

}

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)

        ;
    }
}