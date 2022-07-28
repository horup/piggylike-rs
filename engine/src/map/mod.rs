use bevy::{prelude::*, asset::FileAssetIo};
use tiled::*;
use std::path::PathBuf;

use crate::metadata::{Metadata, Id};

pub fn get_assets_path(world:&World) -> PathBuf {
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let asset_io = asset_server.asset_io().downcast_ref::<FileAssetIo>().unwrap();
    let assets_path = asset_io.root_path().clone();
    return assets_path;
}

pub fn load_map(world:&mut World, map_path:&str) -> Result<()> {
    let mut loader = Loader::new();
    let mut path = get_assets_path(world).clone();
    path.push(PathBuf::from(map_path));
    let map = loader.load_tmx_map(path)?;

    world.clear_entities();

    // create a camera
    let mut camera_entity = world.spawn();
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1.0/16.0;
    camera_entity.insert_bundle(camera_bundle);

    let metadata = world.get_resource::<Metadata>().unwrap().clone();
    // spawn tilemap
    for layer in map.layers() {
        match layer.layer_type() {
            LayerType::TileLayer(tile_layer) => {
                if let (Some(width), Some(height)) = (tile_layer.width(), tile_layer.height()) {
                    for y in 0..map.width {
                        for x in 0..map.height {
                            if let Some(tile) = tile_layer.get_tile(x as i32, y as i32) {
                                let id = tile.id() as Id;
                                let tile_def = metadata.tiles.get(&id).clone();

                                if let Some(tile_def) = tile_def {
                                    println!("{:?}", tile_def.atlas_index);
                                    let atlas_def = metadata.atlases.get(&tile_def.atlas).clone();
                                    if let Some(atlas_def) = atlas_def {
                                        let mut tile = world.spawn();
                                        tile.insert_bundle(SpriteSheetBundle {
                                            sprite:TextureAtlasSprite {
                                                index:tile_def.atlas_index as usize,
                                                custom_size:Some(Vec2::new(1.0, 1.0)),
                                                ..Default::default()
                                            },
                                            texture_atlas: atlas_def.handle.clone(),
                                            transform:Transform {
                                                translation:Vec3::new(x as f32, y as f32, 0.0),
                                                ..Default::default()
                                            },
                                            ..default()
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                
            },
            LayerType::ObjectLayer(object_layer) => {

            },
            _ => {}
        }
    }
    
   /* commands.spawn_bundle(camera_bundle);
    let size = 256;
    for y in 0..size {
        for x in 0..size {
            commands.spawn_bundle(SpriteSheetBundle {
                sprite:TextureAtlasSprite {
                    index:x % 17 as usize,
                    custom_size:Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle.clone(),
                transform:Transform {
                    translation:Vec3::new(x as f32, y as f32, 0.0),
                    ..Default::default()
                },
                ..default()
            });
        }
    } */

    Ok(())
}