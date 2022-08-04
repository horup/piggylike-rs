use bevy::{prelude::{World, Transform, Component, Entity}, sprite::{SpriteSheetBundle, TextureAtlasSprite}, math::{Vec2, Vec3}};
use serde::{Serialize, Deserialize};
use crate::resources::{Id, ThingDef, Metadata};
use super::{Player, Controller, Body};

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Thing {
    pub thing_def:Id,
}

pub fn spawn_thing(world:&mut World, x:f32, y:f32, z:f32, thing_def_id:&Id, metadata:&Metadata) -> Option<Entity> {
    if let Some(thing_def) = metadata.things.get(thing_def_id) {
        let p = Vec3::new(x as f32, y as f32, z as f32);
        let mut e = world.spawn();
        e.insert(Thing {
            thing_def:thing_def_id.clone()
        });
        
        e.insert(Body {
            pos:p,
            size:1.0,
            solid:thing_def.solid,
            ..Default::default()
        });

        if thing_def.player {
            e.insert(Player::default());
            e.insert(Controller::default());
        }

        return Some(e.id());
    }

    return None;
    
}
