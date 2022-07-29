use bevy::prelude::*;

use crate::components::{Controller, Velocity};

pub fn controller_system(mut query:Query<(&Controller, &mut Velocity)>, time:Res<Time>) {
    /*
    
    
     let wish_dir = Vec2::new(self.input.x, self.input.y).normalize();
            let wish_speed = 5.0;
            let accel = 10.00;
            let current_speed = wish_dir.dot(thing.vel);
            let add_speed = wish_speed - current_speed;


            if add_speed > 0.0 {
                let accel_speed = accel * wish_speed * dt;
                let accel_speed = if accel_speed > add_speed {add_speed} else {accel_speed};
                thing.vel += wish_dir * accel_speed;
            }
    
    */
    let dt = time.delta_seconds().min(0.1);
    
    query.for_each_mut(|(controller, mut velocity)| {
        let wish_dir = controller.wish_dir.normalize_or_zero();
        let accel = 10.0;
        let wish_speed = 5.0;
        let current_speed = velocity.velocity.length();
        let add_speed = wish_speed - current_speed;
        if add_speed > 0.0 {
            let accel_speed = accel * wish_speed * dt;
            let accel_speed = if accel_speed > add_speed {add_speed} else {accel_speed};
            velocity.velocity += wish_dir * accel_speed;
        }
    });
}