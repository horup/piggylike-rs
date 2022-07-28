use macroquad::prelude::*;

use crate::Engine;

#[derive(Debug, Clone, Copy, Default)]
pub struct Input {
    pub x:f32,
    pub y:f32,
}

impl Engine {
    pub fn update_input(&mut self) {
        let dt = self.get_delta_time();

        if is_key_down(KeyCode::F1) {
            self.pop_timeline();
        }

        if is_key_pressed(KeyCode::F5) {
            self.save_world("quicksave.sav");
        }
        if is_key_pressed(KeyCode::F6) {
            self.load_world("quicksave.sav");
        }

        self.input = Input {
            x:if is_key_down(KeyCode::A) {-1.0} else if is_key_down(KeyCode::D) {1.0} else {0.0},
            y:if is_key_down(KeyCode::W) {-1.0} else if is_key_down(KeyCode::S) {1.0} else {0.0},
        };


       

        if let Some((_, thing)) = self.world.things.iter_mut().find(|(_, thing)| thing.player) {
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
        }
    }
}