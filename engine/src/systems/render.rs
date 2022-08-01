use bevy::prelude::*;
use crate::resources::Config;

pub fn render_system(mut query:Query<(&mut Transform)>, config:Res<Config>) {
    for (mut transform) in  query.iter_mut() {
        //transform.translation = Config::snap_vec32(transform.translation, 1.0 / 8.0);
    }
}