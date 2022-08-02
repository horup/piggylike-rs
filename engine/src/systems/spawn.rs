use bevy::{prelude::*, sprite::Anchor};
use rune::compile::Meta;
use crate::{components::{Cam, Thing, Tilesprite}, resources::Tilemap, resources::Metadata};

pub fn spawn_camera_system(mut commands:Commands, query:Query<(Entity, Added<Cam>)>) {
    query.for_each(|(e, added)| {
        if added {
            let mut e = commands.entity(e);
            let mut camera_bundle = Camera2dBundle::default();
            e.insert_bundle(camera_bundle);
        }
    });
}

pub fn spawn_tilemap_system(mut commands:Commands, mut tilemap:ResMut<Tilemap>, metadata:Res<Metadata>, tile_sprites:Query<(Entity, &Tilesprite)>) {
    if tilemap.is_changed() {
        tile_sprites.for_each(|(e, _)| {
            commands.entity(e).despawn();
        });

        tilemap.tiles.iter_mut().for_each(|tile| {
            if let Some(tile) = tile {
                tile.entity = None;
            }
        });

        for y in 0..tilemap.height {
            for x in 0..tilemap.width {
                if let Some(tile) = tilemap.get_mut(x as i32, y as i32) {
                    if let Some(tile_def) = metadata.tiles.get(&tile.tile_def) {
                        if let Some(atlas_def) = metadata.atlases.get(&tile_def.atlas) {
                            let mut e = commands.spawn();
                            e.insert_bundle(SpriteSheetBundle {
                                sprite:TextureAtlasSprite {
                                    index:tile_def.atlas_index as usize,
                                    custom_size:Some(Vec2::new(1.0, 1.0)),
                                    anchor:Anchor::BottomLeft,
                                    ..Default::default()
                                },
                                texture_atlas: atlas_def.handle.clone(),
                                transform:Transform {
                                    translation:Vec3::new(x as f32, y as f32, 0.0),
                                    ..Default::default()
                                },
                                ..default()
                            });
                            e.insert(Tilesprite::default());
                            tile.entity = Some(e.id());

                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_things_system(mut commands:Commands, query:Query<(Entity, Added<Thing>, &Thing)>, metadata:Res<Metadata>) {
    query.for_each(|(e, added, thing)| {
        if added {
            if let Some(thing_def) = metadata.things.get(&thing.thing_def) {
                if let Some(atlas_def) = metadata.atlases.get(&thing_def.atlas) {
                    let mut e = commands.entity(e);
                    e.insert_bundle(SpriteSheetBundle {
                        sprite:TextureAtlasSprite {
                            index:thing_def.atlas_index as usize,
                            custom_size:Some(Vec2::new(1.0, 1.0)),
                            ..Default::default()
                        },
                        texture_atlas: atlas_def.handle.clone(),
                        ..Default::default()
                    });
                }
            }
        }
    });
}