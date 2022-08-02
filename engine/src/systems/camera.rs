use bevy::{prelude::*, render::camera::ScalingMode};

use crate::components::Player;

pub fn camera_system(
    mut set: ParamSet<(
        Query<(&Player, &Transform)>,
        Query<(&Camera, &mut Transform, &mut OrthographicProjection)>,
    )>,
    mut windows: ResMut<Windows>
) {
    let window = windows.get_primary_mut().unwrap();
    let width = window.width();
    let height = window.height();
    let mut translation = Vec3::default();
    set.p0()
        .iter()
        .for_each(|(_, transform)| translation = transform.translation.clone());

    set.p1().for_each_mut(|(_, mut transform, mut projection)| {
       // transform.translation = translation;
        //transform.translation.x = transform.translation.x.floor();
        //transform.translation.y = transform.translation.y.floor();
        //let w = projection.left.abs() + projection.right.abs();
        //let h = projection.top.abs() + projection.bottom.abs();
       
        let wish_visible_tiles = 12.0;
        let visible_tiles_horz = width / wish_visible_tiles;
        let f_horz = (visible_tiles_horz / wish_visible_tiles).floor();

        let visible_tiles_vert = height / wish_visible_tiles;
        let f_vert = (visible_tiles_vert / wish_visible_tiles).floor();
        
        let scale = (wish_visible_tiles * f_horz).min(wish_visible_tiles * f_vert);

        projection.scale = 1.0 / scale;
        
        //let scale = 1.0 / 16.0;
        //projection.scale = scale;
       /* let alpha = 1.0/64.0;
        transform.translation.x = (transform.translation.x / alpha).floor() * alpha;
        transform.translation.y = (transform.translation.y / alpha).floor() * alpha;*/
       // projection.scale = 1.0 / scale;
    });
}
