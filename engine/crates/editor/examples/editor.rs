use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use editor::*;
use smart_camera::*;
use tilemap::*;
use metadata::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MetadataPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(SmartCameraPlugin)
        .add_startup_system(setup)
        .run();
}
 


fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut metadata:ResMut<Metadata>) {
    metadata.tiles.insert(0, TileDef {
        solid: false,
        mesh: "../../../assets/bush.glb#Scene0".into(),
    });
    metadata.tiles.insert(1, TileDef {
        solid: false,
        mesh: "../../../assets/grass.glb#Scene0".into(),
    });
    metadata.tiles.insert(2, TileDef {
        solid: false,
        mesh: "../../../assets/stone_wall.glb#Scene0".into(),
    });
    metadata.tiles.insert(3, TileDef {
        solid: false,
        mesh: "../../../assets/stone.glb#Scene0".into(),
    });
    metadata.tiles.insert(4, TileDef {
        solid: false,
        mesh: "../../../assets/stone.glb#Scene0".into(),
    });


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
