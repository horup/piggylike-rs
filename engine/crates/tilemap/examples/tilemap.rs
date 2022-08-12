use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use metadata::{MetadataPlugin, Metadata, TileDef, Id};
use smart_camera::*;
use tilemap::{TilemapPlugin, Tilemap, Tile};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MetadataPlugin)
        .add_plugin(SmartCameraPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .insert_resource(Edit { id:0 })
        .add_system(update_cursor)
        .add_startup_system(setup)
        .run();
}

fn update_cursor(keys:Res<Input<KeyCode>>, mut query:Query<(&mut Transform, &Cursor3D)>, world_cursor:Res<WorldCursor>, mut tilemap:ResMut<Tilemap>, buttons:Res<Input<MouseButton>>, mut edit:ResMut<Edit>) {
    if keys.pressed(KeyCode::Key1) { edit.id = 1 }
    if keys.pressed(KeyCode::Key2) { edit.id = 2 }
    if keys.pressed(KeyCode::Key3) { edit.id = 3 }
    if keys.pressed(KeyCode::Key4) { edit.id = 4 }
    if keys.pressed(KeyCode::Key5) { edit.id = 5 }

    
    query.for_each_mut(|(mut transform, _)| {
        transform.translation.y = 0.5;
        let p = world_cursor.position.clamp(Vec3::new(0.0, 0.0, 0.0), Vec3::new(tilemap.width as f32, 0.0, tilemap.height as f32)).floor();
        transform.translation.x = p.x + 0.5;
        transform.translation.z = p.z + 0.5;

        if buttons.pressed(MouseButton::Left) {
            tilemap.set(p.x as i32, p.z as i32, Some(Tile {
                tile_def:edit.id,
                ..Default::default()
            }));
        } else if buttons.pressed(MouseButton::Right) {
            tilemap.set(p.x as i32, p.z as i32, None);
        }
    });
}

#[derive(Component)]
struct Cursor3D;

#[derive(Default)]
struct Edit {
    pub id:Id
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut metadata: ResMut<Metadata>
) {
    let tilemap = Tilemap::new(16, 16);

    commands.insert_resource(tilemap);


    metadata.tiles.insert(0, TileDef {
        solid: false,
        mesh: "../../../assets/bush.glb#Scene0".into(),
        name: "".into(),
    });
    metadata.tiles.insert(1, TileDef {
        name: "".into(),
        solid: false,
        mesh: "../../../assets/grass.glb#Scene0".into(),
    });
    metadata.tiles.insert(2, TileDef {
        name: "".into(),
        solid: false,
        mesh: "../../../assets/stone_wall.glb#Scene0".into(),
    });
    metadata.tiles.insert(3, TileDef {
        name: "".into(),
        solid: false,
        mesh: "../../../assets/stone.glb#Scene0".into(),
    });
    metadata.tiles.insert(4, TileDef {
        name: "".into(),
        solid: false,
        mesh: "../../../assets/stone.glb#Scene0".into(),
    });


    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // selection
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgba(1.0, 1.0, 1.0, 0.5).into()),
        ..Default::default()
    }).insert(Cursor3D)
    .insert_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // target
    commands
        .spawn_bundle(PbrBundle {
       //     mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
         //   material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(CameraTarget::default())
        .insert(Controller::default());

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(SmartCamera::default());
}
