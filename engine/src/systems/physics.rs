use bevy::{prelude::*, utils::HashMap};
use parry2d::{shape::Cuboid, query::{self, Contact}, na::Isometry2};

use crate::{components::Body, resources::Tilemap, events::{TouchEvent, Touchee}};

pub fn physics_system(mut query:Query<(Entity, &mut Body)>, time:ResMut<Time>, tilemap:Res<Tilemap>, mut touch_events:EventWriter<TouchEvent>) {
    let dt = time.delta_seconds().min(0.1);
    
    let mut cloned_bodies:HashMap<Entity, Body> = query.iter().map(|(entity, body)| (entity, body.clone())).into_iter().collect();
    for ((entity, mut body)) in query.iter_mut(){
        let ground_friction = 10.0;
        let mut total_friction = 0.0;
        
        total_friction += ground_friction;
        body.vel = body.vel - body.vel * total_friction * dt;
      
        if (body.vel.length() < 0.01) {
            body.vel = Vec3::new(0.0, 0.0, 0.0);
        }

        let size = 0.5;
        let thing_shape = Cuboid::new([size * 0.9, size * 0.9].into());
        let tile_shape = Cuboid::new([size, size].into());
        let vels = [Vec3::new(body.vel.x, 0.0, 0.0) * dt, Vec3::new(0.0, body.vel.y, 0.0) * dt];
        let mut contact_entity:Option<Entity> = None;
        let mut contact_tile:Option<IVec2> = None;
    
        for vel in vels {
            if vel.length() > 0.0 {
                let new_pos = body.pos + vel;
                let tiles = Body::get_tiles_in_front(new_pos, [if vel.x > 0.0 {1} else if vel.x < 0.0 {-1} else {0}, if vel.y > 0.0 {1} else if vel.y < 0.0 {-1} else {0}].into());
                
                let mut contact:Option<Contact> = None;
                for tile_index in tiles {
                    let tile_pos = Vec2::new(tile_index.x as f32 + 0.5, tile_index.y as f32 + 0.5);
                    if let Some(tile) = tilemap.get(tile_index.x as i32, tile_index.y as i32) {
                        if tile.solid {
                            let res = query::contact(&Isometry2::translation(new_pos.x, new_pos.y), 
                            &thing_shape, &Isometry2::translation(tile_pos.x, tile_pos.y), &thing_shape, 1.0);
                            if let Ok(Some(res)) = res {
                                if res.dist < 0.0 {
                                    if let Some(c) = contact {
                                        if res.dist < c.dist {
                                            contact = Some(res);
                                            contact_tile = Some(tile_index);
                                        }
                                    } else {
                                        contact = Some(res);
                                        contact_tile = Some(tile_index);
                                    }
                                }
                            }
                        }
                    }
                }

                for (other_id, other_thing) in cloned_bodies.iter().filter(|(other_id, other_thing)| {&&entity != other_id && other_thing.solid}) {
                    let res = query::contact(&Isometry2::translation(new_pos.x, new_pos.y), 
                            &thing_shape, &Isometry2::translation(other_thing.pos.x, other_thing.pos.y), &thing_shape, 1.0);

                    if let Ok(Some(res)) = res {
                        if res.dist < 0.0 {
                            if let Some(c) = contact {
                                if res.dist < c.dist {
                                    contact = Some(res);
                                    contact_entity = Some(other_id.clone());
                                    contact_tile = None;
                                }
                            } else {
                                contact = Some(res);
                            }
                        }
                    }    
                }

                if let Some(contact) = contact {
                    let v = vel.normalize() * contact.dist;
                    body.pos = new_pos + v;
                    if let Some(e) = contact_entity {
                        touch_events.send(TouchEvent {
                            toucher:entity, 
                            touchee:Touchee::Entity((e))
                        });
                    }
                    else if let Some(tile) = contact_tile {
                        touch_events.send(TouchEvent {
                            toucher:entity, 
                            touchee:Touchee::Tile((tile))
                        });
                    }
                } else {
                    body.pos = new_pos;
                }
            }
        }

        cloned_bodies.get_mut(&entity).unwrap().pos = body.pos;
    }
    
    
    /*for (mut body) in query.iter_mut() {
        let ground_friction = 10.0;
        let mut total_friction = 0.0;
        total_friction += ground_friction;
        body.vel = body.vel - body.vel * total_friction * dt;
        body.pos = body.pos + body.vel * dt;
    }*/
}



/*



use generational_arena::Index;
use macroquad::prelude::*;
use parry2d::{bounding_volume::{AABB, BoundingVolume}, shape::{Cuboid, Ball}, query, na::{Isometry2, ComplexField}};

use crate::{Engine, Thing, Command};

impl Engine {
    pub fn update_movement(&mut self) {
        //let mut commands = self.commands.borrow_mut();
        let dt = self.get_delta_time();
        let ground_friction = 0.4;
        // https://www.youtube.com/watch?v=v3zT3Z5apaM
        let alpha = 0.001;
        let mut cloned_things = self.world.things.clone();
        for (id, thing) in self.world.things.iter_mut() {
            let ground_friction = 10.0;
            let mut total_friction = 0.0;
            
            total_friction += ground_friction;
            thing.vel = thing.vel - thing.vel * total_friction * dt;
          
            if (thing.vel.length() < 0.01) {
                thing.vel = Vec2::new(0.0, 0.0);
            }

            let size = 0.5;
            let thing_shape = Cuboid::new([size * 0.9, size * 0.9].into());
            let tile_shape = Cuboid::new([size, size].into());
            let vels = [Vec2::new(thing.vel.x, 0.0) * dt, Vec2::new(0.0, thing.vel.y) * dt];
            let mut contact_index:Option<Index> = None;
        
            for vel in vels {
                if vel.length() > 0.0 {
                    let new_pos = thing.pos + vel;
                    let tiles = Thing::get_tiles_in_front(new_pos, [if vel.x > 0.0 {1} else if vel.x < 0.0 {-1} else {0}, if vel.y > 0.0 {1} else if vel.y < 0.0 {-1} else {0}].into());
                    
                    let mut contact:Option<query::Contact> = None;
                    for tile in tiles {
                        let tile_pos = Vec2::new(tile.x as f32 + 0.5, tile.y as f32 + 0.5);
                        if let Some(tile) = self.world.tilemap.get(0, tile.x as i32, tile.y as i32) {
                            if tile.solid {
                                let res = query::contact(&Isometry2::translation(new_pos.x, new_pos.y), 
                                &thing_shape, &Isometry2::translation(tile_pos.x, tile_pos.y), &thing_shape, 1.0);

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

                    for (other_id, other_thing) in cloned_things.iter().filter(|(other_id, other_thing)| {&id != other_id && other_thing.solid}) {
                        let res = query::contact(&Isometry2::translation(new_pos.x, new_pos.y), 
                                &thing_shape, &Isometry2::translation(other_thing.pos.x, other_thing.pos.y), &thing_shape, 1.0);

                        if let Ok(Some(res)) = res {
                            if res.dist < 0.0 {
                                if let Some(c) = contact {
                                    if res.dist < c.dist {
                                        contact = Some(res);
                                        contact_index = Some(other_id);
                                    }
                                } else {
                                    contact = Some(res);
                                }
                            }
                        }    
                    }

                    if let Some(contact) = contact {
                        let v = vel.normalize() * contact.dist;
                        thing.pos = new_pos + v;
                        thing.touched_thing = contact_index;
                    } else {
                        thing.pos = new_pos;
                    }
                }
            }

            if thing.player {
                self.world.camera.pos = thing.pos;
            }

            *cloned_things.get_mut(id).unwrap().pos = *thing.pos;

        }
    }
}




*/