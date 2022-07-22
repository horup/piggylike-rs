use macroquad::prelude::*;
use parry2d::{bounding_volume::{AABB, BoundingVolume}, shape::Cuboid, query, na::{Isometry2, ComplexField}};

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
                    let cube1 = Cuboid::new([size, size].into());
                    let cube2 = Cuboid::new([size, size].into());
                    let mut contact:Option<query::Contact> = None;
                    for tile in tiles {
                        let tile_pos = Vec2::new(tile.x as f32 + 0.5, tile.y as f32 + 0.5);
                        if let Some(tile) = self.world.tilemap.get(0, tile.x as i32, tile.y as i32) {
                            if tile.solid {
                                let res = query::contact(&Isometry2::translation(new_pos.x, new_pos.y), 
                                &cube1, &Isometry2::translation(tile_pos.x, tile_pos.y), &cube1, 1.0);

                                if let Ok(Some(res)) = res {
                                    if res.dist < 0.0 {
                                        if let Some(c) = contact {
                                            if res.dist < c.dist {
                                                contact = Some(res);
                                            }
                                        } else {
                                            contact = Some(res);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(contact) = contact {
                        let v = vel.normalize() * contact.dist;
                        thing.pos = new_pos + v;
                    } else {
                        thing.pos = new_pos;
                    }
                }
            }

            if thing.player {
                self.world.camera.pos = thing.pos;
            }
        }
    }
}
