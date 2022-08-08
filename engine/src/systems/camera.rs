 use bevy::{prelude::*, render::camera::ScalingMode};

use crate::components::Player;

pub fn camera_system(
    mut set: ParamSet<(
        Query<(&Player, &Transform)>,
        Query<(&Camera, &mut Transform)>,
    )>,
    mut windows: ResMut<Windows>,
) {
  /*  if let Some(window) = windows.get_primary_mut() {
        let width = window.width();
        let height = window.height();
        let mut target = Vec3::default();
        set.p0()
            .iter()
            .for_each(|(_, transform)| target = transform.translation.clone());

        set.p1().for_each_mut(|(_, mut transform)| {
            let cam_distance = 16.0;
            let dist = Vec3::new(0.0, 1.0, 1.0).normalize_or_zero() * cam_distance;
            transform.translation = target + dist;
            *transform = transform.looking_at(target, Vec3::Y);
        });
    }*/
}
