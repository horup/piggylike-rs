use bevy::prelude::*;
use crate::{components::Body, events::{TouchEvent, Touchee}};

pub fn proximity_system(mut bodies:Query<(Entity, &Body)>, mut touch_events:EventWriter<TouchEvent>) {
    let clones:Vec<(Entity, Body)> = bodies.iter().map(|(entity, body)| (entity, body.clone())).collect();
    for (entity, body) in bodies.iter() {
        for (other_entity, other_body) in clones.iter().filter(|(other_entity, other_body)| other_entity != &entity && other_body.solid == false) {
            let pos1 = body.pos;
            let pos2 = other_body.pos;

            let l = body.size + other_body.size;
            let v:Vec3 = pos1 - pos2;
            if v.length() < l / 2.0 {
                touch_events.send(TouchEvent {
                    toucher:entity,
                    touchee:Touchee::Entity(*other_entity)
                });
            }
        }
    }
}