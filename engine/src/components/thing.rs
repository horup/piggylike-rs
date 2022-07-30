use bevy::{prelude::{World, Transform, Component, Entity}, sprite::{SpriteSheetBundle, TextureAtlasSprite}, math::{Vec2, Vec3}};
use serde::{Serialize, Deserialize};
use crate::metadata::{Id, ThingDef, Metadata};
use super::{Player, Controller, Body};

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Thing {
    pub thing_def:Id,
}

pub fn spawn_thing(world:&mut World, x:f32, y:f32, thing_def_id:&Id, metadata:&Metadata) -> Option<Entity> {
    if let Some(thing_def) = metadata.things.get(thing_def_id) {
        let atlas = thing_def.atlas as Id;
        if let Some(atlas_def) = metadata.atlases.get(&atlas) {
            let p = Vec3::new(x as f32, y as f32, 0.0);
            let mut e = world.spawn();
            e.insert(Thing {
                thing_def:thing_def_id.clone()
            });
            e.insert_bundle(SpriteSheetBundle {
                sprite:TextureAtlasSprite {
                    index:thing_def.atlas_index as usize,
                    custom_size:Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                texture_atlas: atlas_def.handle.clone(),
                transform:Transform {
                    translation:Vec3::new(x as f32, y as f32, 0.0),
                    ..Default::default()
                },
                ..Default::default()
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
    }

    return None;
    
}
