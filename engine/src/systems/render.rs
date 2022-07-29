use bevy::prelude::*;

use crate::components::Body;


pub fn render_system(mut query:Query<(&Body, &mut Transform)>) {
    let r = 1.0/10.0;
    for (body, mut transform) in query.iter_mut() {
        for i in 0..3 {
            transform.translation[i] = (body.position[i] / r).floor() * r;
        }
    }
}