use bevy::prelude::*;
use smart_camera::*;
use tilemap::*;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands
        .spawn_bundle(Camera3dBundle {
            ..Default::default()
        })
        .insert(SmartCamera::default());

    commands
        .spawn()
        .insert(Transform::default())
        .insert(CameraTarget::default())
        .insert(Controller::default());

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Grid { size: 16 })),
        ..Default::default()
    });
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SmartCameraPlugin)
        .add_startup_system(setup)
        .run();
}
