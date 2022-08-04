use bevy::{prelude::*, sprite::Anchor};
use rune::compile::Meta;
use crate::{components::{Cam, Thing, Tilesprite}, resources::Tilemap, resources::Metadata};

pub fn spawn_camera_system(mut commands:Commands, query:Query<(Entity, Added<Cam>)>) {
    query.for_each(|(e, added)| {
        if added {
            let mut e = commands.entity(e);
            e.insert_bundle(Camera3dBundle {
                ..Default::default()
            });

            commands.insert_resource(AmbientLight { brightness:0.1, ..Default::default() });
        }
    });
}

pub fn spawn_tilemap_system(mut commands:Commands, mut meshes:ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,asset_server:Res<AssetServer>, mut tilemap:ResMut<Tilemap>, metadata:Res<Metadata>, tile_sprites:Query<(Entity, &Tilesprite)>) {
    if tilemap.is_changed() {
        tile_sprites.for_each(|(e, _)| {
            commands.entity(e).despawn();
        });

        tilemap.tiles.iter_mut().for_each(|tile| {
            if let Some(tile) = tile {
                tile.entity = None;
            }
        });

        let mat = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
        let floor = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));
        let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
        for y in 0..tilemap.height {
            for x in 0..tilemap.width {
                if let Some(tile) = tilemap.get_mut(x as i32, y as i32) {
                    if let Some(tile_def) = metadata.tiles.get(&tile.tile_def) {
                        if let Some(atlas_def) = metadata.atlases.get(&tile_def.atlas) {
                            let mut e = commands.spawn();
                            if tile_def.solid {
                               /* e.insert_bundle(PbrBundle {
                                    mesh: cube.clone(),
                                    material: mat.clone(),
                                    transform: Transform::from_xyz(x as f32 + 0.5, 0.5, y as f32 + 0.5),
                                    ..default()
                                });*/
                                e.insert_bundle(SceneBundle {
                                    scene:asset_server.load("meshes/wall.glb#Scene0"),
                                    transform: Transform::from_xyz(x as f32 + 0.5, 0.0, y as f32 + 0.5),
                                    ..Default::default()
                                });
                            } else {
                                e.insert_bundle(PbrBundle {
                                    mesh: floor.clone(),
                                    material: mat.clone(),
                                    transform: Transform::from_xyz(x as f32 + 0.5, 0.0, y as f32 + 0.5),
                                    ..default()
                                });
                            }

                            tile.entity = Some(e.id());
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_things_system(mut commands:Commands, asset_server:Res<AssetServer>, query:Query<(Entity, Added<Thing>, &Thing)>, metadata:Res<Metadata>) {
    query.for_each(|(e, added, thing)| {
        if added {
            if let Some(thing_def) = metadata.things.get(&thing.thing_def) {
                if let Some(atlas_def) = metadata.atlases.get(&thing_def.atlas) {
                    let mut e = commands.entity(e);
                   /* e.insert_bundle(SpriteSheetBundle {
                        sprite:TextureAtlasSprite {
                            index:thing_def.atlas_index as usize,
                            custom_size:Some(Vec2::new(1.0, 1.0)),
                            ..Default::default()
                        },
                        texture_atlas: atlas_def.handle.clone(),
                        ..Default::default()
                    });*/

                    e.insert_bundle(SceneBundle {
                        scene:asset_server.load("meshes/piggy.glb#Scene0"),
                        transform:Transform {
                            //translation:Vec3::new(x as f32, 0.0, y as f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                }
            }
        }
    });
}