use macroquad::prelude::*;

use crate::Engine;

impl Engine {
    pub fn update_movement(&mut self) {
        let dt = self.get_delta_time();
        let ground_friction = 0.4;
        // https://www.youtube.com/watch?v=v3zT3Z5apaM
        let alpha = 0.001;
        for (_, thing) in self.world.things.iter_mut() {
            thing.vel = thing.vel * ground_friction; // missing dt
            if (thing.vel.length() < 0.01) {
                thing.vel = Vec2::new(0.0, 0.0);
            }
            thing.pos += thing.vel;

            if thing.player {
                self.world.camera.pos = thing.pos;
            }
        }

        
        
    }
}