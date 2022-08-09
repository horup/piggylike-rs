use core::bevy::ecs as bevy_ecs;
use core::bevy::prelude::*;

#[derive(Component, Clone, Copy, Default)]
pub struct SmartCamera {
    pub target:Vec3
}

#[derive(Component, Clone, Copy, Default)]
pub struct SmartCameraTarget {

}

pub fn smart_camera_spawn(mut commands:Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(SmartCamera {
        ..Default::default()
    });
}

pub fn smart_camera_update() {

}