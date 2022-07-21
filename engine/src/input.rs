use macroquad::prelude::*;

use crate::Engine;

#[derive(Debug, Clone, Copy, Default)]
pub struct Input {
    pub x:f32,
    pub y:f32
}

impl Engine {
    pub fn update_input(&mut self) {
        let dt = self.get_delta_time();
        self.input = Input {
            x:if is_key_down(KeyCode::A) {-1.0} else if is_key_down(KeyCode::D) {1.0} else {0.0},
            y:if is_key_down(KeyCode::W) {-1.0} else if is_key_down(KeyCode::S) {1.0} else {0.0},
        };

        if let Some((_, player)) = self.world.things.iter_mut().find(|(_, thing)| thing.player) {
            let acc = 10.0;
            player.vel.x += self.input.x * acc * dt;
            player.vel.y += self.input.y * acc * dt;
        }
    }
}