mod player;
use bevy::{prelude::{World, Transform}, sprite::{SpriteSheetBundle, TextureAtlasSprite}, math::{Vec2, Vec3}};
pub use player::*;

mod body;
pub use body::*;

mod controller;
pub use controller::*;

use crate::metadata::{ThingDef, Metadata, Id};

pub fn spawn_thing(world:&mut World, x:f32, y:f32, thing_def:&ThingDef, metadata:&Metadata) {
    let atlas = thing_def.atlas as Id;
    if let Some(atlas_def) = metadata.atlases.get(&atlas) {
        let mut e = world.spawn();
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
    }
}