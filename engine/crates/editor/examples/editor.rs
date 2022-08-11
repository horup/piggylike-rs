use bevy::prelude::*;
use smart_camera::*;

fn setup(mut commands:Commands) {
    /*commands.spawn_bundle(Camera3dBundle {
        ..Default::default()
    });*/
}

pub fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(SmartCameraPlugin)


    .add_startup_system(setup)
    .run()

    ;
}