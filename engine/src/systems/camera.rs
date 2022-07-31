use bevy::prelude::*;

use crate::components::Player;

pub fn camera_system(
    mut set: ParamSet<(
        Query<(&Player, &Transform)>,
        Query<(&Camera, &mut Transform, &mut OrthographicProjection)>,
    )>,
) {
    let mut translation = Vec3::default();
    set.p0()
        .iter()
        .for_each(|(_, transform)| translation = transform.translation.clone());

    set.p1().for_each_mut(|(_, mut transform, mut projection)| {
        transform.translation = translation;

        let w = projection.left.abs() + projection.right.abs();
        let h = projection.top.abs() + projection.bottom.abs();
       
        let visible_tiles = 16.0;

        let scale = w / visible_tiles;
       /* let alpha = 1.0/64.0;
        transform.translation.x = (transform.translation.x / alpha).floor() * alpha;
        transform.translation.y = (transform.translation.y / alpha).floor() * alpha;*/
        projection.scale = 1.0 / scale;
    });
}
