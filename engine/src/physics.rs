use macroquad::prelude::*;
use parry2d::{bounding_volume::{AABB, BoundingVolume}, shape::Cuboid};

use crate::{Engine, Thing};

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

            let vels = [Vec2::new(thing.vel.x, 0.0), Vec2::new(0.0, thing.vel.y)];

            for vel in vels {
                if vel.length() > 0.0 {
                    let new_pos = thing.pos + vel;
                    let tiles = Thing::get_tiles_in_front(new_pos, [if vel.x > 0.0 {1} else if vel.x < 0.0 {-1} else {0}, if vel.y > 0.0 {1} else if vel.y < 0.0 {-1} else {0}].into());
                    let size = 0.5;
                    let thing_aabb = AABB::new([new_pos.x - size, new_pos.y - size].into(), [new_pos.x + size, new_pos.y + size].into());
                    let mut collided = false;
                    for tile in tiles {
                        let tile_aabb = AABB::new([tile.x as f32, tile.y as f32].into(), [tile.x as f32 + 1.0, tile.y as f32 + 1.0].into());
                        if let Some(tile) = self.world.tilemap.get(0, tile.x as u32, tile.y as u32) {
                            if tile.solid {
                                if thing_aabb.intersects(&tile_aabb) {
                                    collided = true;
                                    break;
                                }
                            }
                        }
                    }

                    if collided == false {
                        thing.pos = new_pos;
                    }
                }
            }


          /*  if (thing.vel.y != 0.0) {
                let mut collision = false;
                if thing.vel.y < 0.0 {
                    let tiles = thing.get_tiles_top();
                    
                    for tile in tiles {
                        let p1 = Vec2::new(tile.x as f32, tile.y as f32);
                        let p2 = thing.pos - Vec2::new(0.5, 0.5);
                        if let Some(tile) = self.world.tilemap.get(0, tile.x as u32, tile.y as u32) {
                            if tile.solid {
                                if Self::collision_test(p1, 1.0, p2, 1.0) {
                                    collision = true;
                                }
                            }
                        }
                    }
                }

                if collision == false {
                    thing.pos.y += thing.vel.y;
                }
            }*/

//            thing.pos += thing.vel;

            if thing.player {
                self.world.camera.pos = thing.pos;
            }
        }

        
        
    }
}