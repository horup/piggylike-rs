use bevy::prelude::*;
use crate::resources::*;
use crate::components::Body;

pub fn interpolation_system(mut query:Query<(&Body, &mut Transform)>) {
    for (body, mut transform) in query.iter_mut() {
        //transform.translation = body.pos;
        transform.translation.x = body.pos.x;
        transform.translation.z = body.pos.y;
    }
}