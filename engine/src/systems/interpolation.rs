use bevy::prelude::*;
use crate::resources::*;
use crate::components::Body;

pub fn interpolation_system(mut query:Query<(&Body, &mut Transform)>) {
    for (body, mut transform) in query.iter_mut() {
        transform.translation = body.pos;
        let mut r = Quat::from_rotation_y(body.facing);
        transform.rotation = r;
    }
}