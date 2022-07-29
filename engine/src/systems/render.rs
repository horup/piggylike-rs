use bevy::prelude::*;
use crate::resources::Config;

pub fn render_system(mut query:Query<(&mut Transform)>, config:Res<Config>) {
    for (mut transform) in  query.iter_mut() {
        transform.translation = config.snap_vec3(transform.translation);
    }
}