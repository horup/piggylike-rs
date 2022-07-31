use bevy::{prelude::*, sprite::Anchor};
use rune::compile::Meta;
use crate::{components::Cam, resources::Tilemap, metadata::Metadata};

pub fn spawn_camera(mut commands:Commands, query:Query<(Entity, Added<Cam>)>) {
    query.for_each(|(e, added)| {
        if added {
            let mut e = commands.entity(e);
            let mut camera_bundle = OrthographicCameraBundle::new_2d();
            camera_bundle.orthographic_projection.scale = 1.0/16.0;
            e.insert_bundle(camera_bundle);
        }
    });
}

pub fn spawn_tilemap(mut commands:Commands, mut tilemap:ResMut<Tilemap>, metadata:Res<Metadata>) {
    if tilemap.is_changed() {
        let mut index = 0;
        tilemap.tiles.iter_mut().for_each(|tile| {
            if let Some(tile) = tile {
                if let Some(entity) = tile.entity {
                    commands.entity(entity).despawn();
                }

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
                            tile.entity = Some(e.id());

                        }
                    }
                }
            }
        }
        //for y in tilemap
      /*  tilemap.tiles.iter_mut().for_each(|tile| {
            if let Some(tile) = tile {
                if let Some(entity) = tile.entity {
                    commands.entity(entity).despawn();
                }

                tile.entity = None;

                let e = commands.spawn();
                if let Some(tile_def) = metadata.tiles.get(&tile.tile_def) {
                    if let Some(atlas_def) = metadata.atlases.get(&tile_def.atlas) {
                        e.insert_bundle(SpriteSheetBundle {
                            sprite:TextureAtlasSprite {
                                index:tile_def.atlas_index as usize,
                                custom_size:Some(Vec2::new(1.0, 1.0)),
                                anchor:Anchor::BottomLeft,
                                ..Default::default()
                            },
                            texture_atlas: atlas_def.handle.clone(),
                            transform:Transform {
                                translation:Vec3::new(wx as f32, wy as f32, 0.0),
                                ..Default::default()
                            },
                            ..default()
                        });
                    }

                    tile.entity = Some(e.id());

                }

                index += 1;
            }
        })*/
    }
}