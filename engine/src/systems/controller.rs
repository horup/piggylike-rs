use bevy::prelude::*;

use crate::components::{Controller, Body};

pub fn controller_system(mut query:Query<(&Controller, &mut Body)>, time:Res<Time>) {
    let dt = time.delta_seconds().min(0.1);
    query.for_each_mut(|(controller, mut velocity)| {
        let wish_dir = controller.wish_dir.normalize_or_zero();
        let accel = 10.0;
        let wish_speed = 5.0;
        let current_speed = velocity.vel.length();
        let add_speed = wish_speed - current_speed;
        if add_speed > 0.0 {
            let accel_speed = accel * wish_speed * dt;
            let accel_speed = if accel_speed > add_speed {add_speed} else {accel_speed};
            velocity.vel += wish_dir * accel_speed;
        }
    });
}