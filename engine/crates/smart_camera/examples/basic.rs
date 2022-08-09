use core::bevy::prelude::*;
use smart_camera::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(smart_camera_spawn)
        .add_system(smart_camera_update)
        .add_system(input)
        .add_startup_system(setup)
        .run();
}

fn input(mut query:Query<(&mut Transform, &SmartCameraTarget)>, input: Res<Input<KeyCode>>, time:Res<Time>) {
    let mut v = Vec3::default();
    let speed = 1.0;
    if input.pressed(KeyCode::A) {v.x -= speed}
    if input.pressed(KeyCode::D) {v.x += speed}
    if input.pressed(KeyCode::W) {v.z -= speed}
    if input.pressed(KeyCode::S) {v.z += speed}

    let v = v.normalize_or_zero() * time.delta_seconds();

    query.for_each_mut(|(mut t,_)| {
        t.translation += v;
    });
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(5.0, 0.5, 0.0),
        ..default()
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

    // center
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert(SmartCameraTarget::default());
}