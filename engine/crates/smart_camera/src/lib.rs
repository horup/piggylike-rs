use core::bevy::ecs as bevy_ecs;
use core::bevy::input::mouse::{MouseWheel, MouseMotion};

use core::bevy::prelude::*;

use std::f32::consts::PI;
 
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

#[derive(Default, Clone, Copy)]
pub struct WorldCursor {
    pub position:Vec3
}

fn cursor_position(query: Query<(&GlobalTransform, &Camera)>, mut cursor_evr: EventReader<CursorMoved>, windows:Res<Windows>, mut cursor_world_pos:ResMut<WorldCursor>) {
    query.for_each(|(transform, camera)| {
        let mut cursor_position = match cursor_evr.iter().last() {
            Some(v) => v.position,
            None => return,
        };

        let window = windows.primary();
        cursor_position.x = cursor_position.x / window.width() * 2.0 - 1.0;
        cursor_position.y = cursor_position.y / window.height() * 2.0 - 1.0;
        let ndc = Vec3::new(cursor_position.x, cursor_position.y, 1.0);

        
     
        let ndc_to_world: Mat4 = camera.projection_matrix() * transform.compute_matrix().inverse();
        let ndc_to_world = ndc_to_world.inverse();
        let p: Vec3 = ndc_to_world.project_point3(ndc);

        let dir: Vec3 = (p - transform.translation()).normalize_or_zero();

        let normal = Vec3::new(0.0, 1.0, 0.0);

        let d = Vec3::new(0.0, 0.0, 0.0).dot(-normal);
        let dir_dot_normal = dir.dot(normal);
        if dir_dot_normal.abs() > 0.001 {
            let t = -(d + transform.translation().dot(normal) / dir_dot_normal);
            let ndc_to_world = transform.translation() + t * dir;
            cursor_world_pos.position = ndc_to_world;
        }
    });
}

fn input(_time:Res<Time>, mut query: Query<(&mut Transform, &mut SmartCamera)>, buttons: Res<Input<MouseButton>>, mut scroll_evr: EventReader<MouseWheel>, mut motion_evr: EventReader<MouseMotion>) {
    let scroll_speed = 0.1;
    let rotate_speed = 0.01;
    query.for_each_mut(|(mut transform, mut camera)| {
        for ev in scroll_evr.iter() {
            camera.distance -= ev.y * scroll_speed * camera.distance;
        }

        camera.distance = camera.distance.clamp(camera.min_distance, camera.max_distance);
        if buttons.pressed(MouseButton::Right) {
            for ev in motion_evr.iter() {
                transform.rotate_y(-ev.delta.x * rotate_speed);
                transform.rotate_local_x(-ev.delta.y * rotate_speed);
                

                let sign = transform.forward().y.signum();
                let angle = transform.up().angle_between(Vec3::Y);
                let max = PI / 2.0;
                let _min = 0.0;
                if angle > max {
                    transform.rotate_local_x(angle - max);
                }

                if sign == 1.0 {
                    transform.look_at(camera.target, Vec3::Y);
                }
            }
        }
    });
    
}

fn find_target(mut cameras:Query<&mut SmartCamera>, targets:Query<(&Transform, &SmartCameraTarget)>) {
    cameras.for_each_mut(|mut camera| {
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
        app.insert_resource(WorldCursor::default());
        app.add_system(cursor_position);
        app.add_system(translate);
        app.add_system(find_target.before(input));
        app.add_system(input.before(translate));
    }
}
