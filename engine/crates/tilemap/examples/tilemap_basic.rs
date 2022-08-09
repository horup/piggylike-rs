use core::bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
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
        .add_startup_system(setup)
        .run();
}

fn move_target(mut query:ParamSet<(Query<(&mut Transform, &SmartCameraTarget)>, Query<(&Transform, &SmartCamera)>)>, input: Res<Input<KeyCode>>, time:Res<Time>) {
    let mut v = Vec3::default();
    let mut up = Vec3::default();
    let speed = 2.0;
    if input.pressed(KeyCode::A) {v.x -= 1.0}
    if input.pressed(KeyCode::D) {v.x += 1.0}
    if input.pressed(KeyCode::W) {v.z -= 1.0}
    if input.pressed(KeyCode::S) {v.z += 1.0}
    if input.pressed(KeyCode::Space) {up.y += 1.0}
    if input.pressed(KeyCode::LShift) {up.y -= 1.0}

    let v = v.normalize_or_zero();

    match query.p1().get_single() {
        Ok((transform, _)) => {
            let transform = transform.clone();
            query.p0().for_each_mut(|(mut t,_)| {
                let v = transform.rotation * v;
                t.translation += Vec3::new(v.x, 0.0, v.z) * speed * time.delta_seconds();
                t.translation += up * speed * speed * time.delta_seconds();
            });
        },
        Err(_) => {},
    }
    
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

   /* let mat = materials.add(Color::rgb(0.3, 0.5, 0.3).into());
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 5.0 }));
    let size = 256;
    for y in 0..size {
        for x in 0..size {
            let p = Vec3::new(x as f32, 0.0, y as f32);
            commands.spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: mat.clone(),
                transform:Transform {
                    translation:p,
                    ..Default::default()
                },
                ..default()
            });
        }
    }*/

    commands.spawn_bundle(PbrBundle {
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        mesh:meshes.add(Mesh::from(tilemap::Grid {
            size:16
        })),
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

    // target
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert(SmartCameraTarget::default());


    commands.spawn_bundle(Camera3dBundle{
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(SmartCamera::default());
}