use core::bevy::ecs as bevy_ecs;
use core::bevy::input::mouse::{MouseWheel, MouseMotion};
use core::bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct SmartCamera {
    pub target: Vec3,
    pub distance: f32,
    pub min_distance: f32,
    pub max_distance: f32,
}

impl Default for SmartCamera {
    fn default() -> Self {
        Self {
            target: Vec3::default(),
            distance: 5.0,
            min_distance: 0.1,
            max_distance: 10.0,
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct SmartCameraTarget {}


fn input(time:Res<Time>, mut query: Query<(&mut Transform, &mut SmartCamera)>, buttons: Res<Input<MouseButton>>, mut scroll_evr: EventReader<MouseWheel>, mut motion_evr: EventReader<MouseMotion>) {
    let scroll_speed = 20.0;
    let rotate_speed = 1.0;
    let dt = time.delta_seconds();
    query.for_each_mut(|(mut transform, mut camera)| {
        for ev in scroll_evr.iter() {
            camera.distance -= ev.y * dt * scroll_speed * camera.distance;
        }

        camera.distance = camera.distance.clamp(camera.min_distance, camera.max_distance);
        if buttons.pressed(MouseButton::Right) {
            for ev in motion_evr.iter() {
                transform.rotate_y(-ev.delta.x * dt);
                transform.rotate_local_x(-ev.delta.y * dt);
                //transform.looking_at(target, up)
            }
        }
    });
    
}

fn find_target(mut cameras:Query<(&mut SmartCamera)>, targets:Query<(&Transform, &SmartCameraTarget)>) {
    cameras.for_each_mut(|(mut camera)| {
        targets.for_each(|(transform, _)| {
            camera.target = transform.translation;
        });
    });
}

fn translate(mut cameras:Query<(&mut Transform, &SmartCamera)>) {
    cameras.for_each_mut(|(mut transform, smart_camera)| {
        let v = transform.rotation * Vec3::Z;
        transform.translation = smart_camera.target + v * smart_camera.distance;
    });
}


pub struct SmartCameraPlugin;

impl Plugin for SmartCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(translate);
        app.add_system(find_target.before(input));
        app.add_system(input.before(translate));
    }
}
