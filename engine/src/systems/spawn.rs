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

           /* commands.spawn_bundle(DirectionalLightBundle {
                transform:Transform {
                    translation: Vec3::new(0.0, 16.0, 16.0),
                    rotation: Quat::from_rotation_y(0.5),
                    ..Default::default()
                },
                ..Default::default()
            });*/
            commands.insert_resource(AmbientLight { brightness:0.80, ..Default::default() });
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
                        if tile_def.mesh.len() == 0 {
                            continue;;
                        }
                        let mut e = commands.spawn();
                            e.insert_bundle(SceneBundle {
                                scene:asset_server.load(&tile_def.mesh),
                                transform: Transform::from_xyz(x as f32 + 0.5, 0.0, y as f32 + 0.5),
                                ..Default::default()
                            });

                        tile.entity = Some(e.id());
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
                    if thing_def.mesh.len() == 0 {
                        return;
                    }

                    let mut e = commands.entity(e);
                    e.insert_bundle(SceneBundle {
                        scene:asset_server.load(&thing_def.mesh),
                        transform:Transform {
                            //translation:Vec3::new(x as f32, 0.0, y as f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                }
        }
    });
}