use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy::ecs as bevy_ecs;
use smart_camera::*;
use tilemap::TilemapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SmartCameraPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_system_to_stage(CoreStage::PreUpdate, move_target)
        .add_system(update_cursor)
        .add_startup_system(setup)
        .run();
}

fn update_cursor(mut query:Query<(&mut Transform, &Cursor3D)>, world_cursor:Res<WorldCursor>) {
    query.for_each_mut(|(mut transform, _)| {
        transform.translation.y = 0.5;
        transform.translation.x = world_cursor.position.x.floor() + 0.5;
        transform.translation.z = world_cursor.position.z.floor() + 0.5;
    });
}

fn move_target(
    mut query: ParamSet<(
        Query<(&mut Transform, &SmartCameraTarget)>,
        Query<(&Transform, &SmartCamera)>,
    )>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut v = Vec3::default();
    let mut up = Vec3::default();
    let speed = 2.0;
    if input.pressed(KeyCode::A) {
        v.x -= 1.0
    }
    if input.pressed(KeyCode::D) {
        v.x += 1.0
    }
    if input.pressed(KeyCode::W) {
        v.z -= 1.0
    }
    if input.pressed(KeyCode::S) {
        v.z += 1.0
    }
    if input.pressed(KeyCode::Space) {
        up.y += 1.0
    }
    if input.pressed(KeyCode::LShift) {
        up.y -= 1.0
    }

    let v = v.normalize_or_zero();

    match query.p1().get_single() {
        Ok((transform, _)) => {
            let transform = transform.clone();
            query.p0().for_each_mut(|(mut t, _)| {
                let v = transform.rotation * v;
                t.translation += Vec3::new(v.x, 0.0, v.z) * speed * time.delta_seconds();
                t.translation += up * speed * speed * time.delta_seconds();
            });
        }
        Err(_) => {}
    }
}

#[derive(Component)]
struct Cursor3D;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        mesh: meshes.add(Mesh::from(tilemap::Grid { size: 256 })),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
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
        .insert(SmartCameraTarget::default());

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(SmartCamera::default());
}
