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
 


fn setup(mut commands: Commands, _meshes: ResMut<Assets<Mesh>>, mut metadata:ResMut<Metadata>) {
    metadata.tiles.insert(0, TileDef {
        name: "Bush Wall".into(),
        solid: false,
        mesh: "../../../assets/bush.glb#Scene0".into(),
    });
    metadata.tiles.insert(1, TileDef {
        name: "Grass Floor".into(),
        solid: false,
        mesh: "../../../assets/grass.glb#Scene0".into(),
    });
    metadata.tiles.insert(2, TileDef {
        name: "Stone Wall".into(),
        solid: false,
        mesh: "../../../assets/stone_wall.glb#Scene0".into(),
    });
    metadata.tiles.insert(3, TileDef {
        name: "Stone Floor".into(),
        solid: false,
        mesh: "../../../assets/stone.glb#Scene0".into(),
    });
    metadata.tiles.insert(4, TileDef {
        name: ".".into(),
        solid: false,
        mesh: "../../../assets/stone.glb#Scene0".into(),
    });


    commands
        .spawn_bundle(Camera3dBundle {
            transform:Transform::from_xyz(8.0, 8.0, 16.0).looking_at(Vec3::new(8.0, 0.0, 8.0), Vec3::Y),
            ..Default::default()
        })
        .insert(SmartCamera::default());

    commands
        .spawn()
        .insert(Transform::from_xyz(8.0, 8.0, 16.0))
        .insert(CameraTarget::default())
        .insert(Controller::default());

   
}
