use bevy::prelude::*;

use crate::components::Player;

pub fn camera_system(
    mut set: ParamSet<(Query<(&Camera, &mut Transform)>, Query<(&Player, &Transform)>)>,
) {
    let mut translation = Vec3::default();
    set.p1().iter().for_each(|(_, transform)| translation = transform.translation.clone());
    set.p0().for_each_mut(|(_, mut transform)| {
        transform.translation = translation;

        let alpha = 1.0/16.0;
        let t = transform.translation;
        transform.translation = Vec3::new((t.x / alpha).floor() * alpha, (t.y / alpha).floor() * alpha, t.z)
    });
}
