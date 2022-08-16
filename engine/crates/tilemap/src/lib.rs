use bevy::prelude::{*, shape::{Cube, Plane}};

mod shape;
use ndarray::Array2;
pub use shape::*;


use bevy::{prelude::Entity};
use serde::{Serialize, Deserialize};

use metadata::{Id, Metadata};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct Tile {
    pub solid:bool,
    pub floor:Option<Id>,
    pub walls:Option<Id>,
    #[serde(skip_serializing)]
    pub entity:Option<Entity>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tilemap {
    pub width:usize,
    pub height:usize,
    pub tiles:Array2<Tile>
}


impl Tilemap {
    pub fn new(width:usize, height:usize) -> Self {
        Self { width, height, tiles: Array2::default((width, height)), }
    }
}

#[derive(Component)]
struct TileEntity {
    pub x:usize,
    pub y:usize,
    pub floor:Option<Id>,
    pub walls:Option<Id>
}

#[derive(Component)]
struct Floor;

#[derive(Component)]
struct Walls;

fn tilemap_changed(mut commands:Commands,asset_server:Res<AssetServer>, mut tilemap:ResMut<Tilemap>, mut metadata:ResMut<Metadata>, _tile_sprites:Query<(Entity, &TileEntity)>) {
    if tilemap.is_changed() {
        for ((x, y), tile) in tilemap.tiles.indexed_iter_mut() {
            if tile.entity == None {
                tile.entity = Some(commands.spawn().with_children(|f|{
                    f.spawn().insert(Floor);
                    f.spawn().insert(Walls);
                }).id());
            }

            let mut entity = commands.entity(tile.entity.unwrap());
            if let Some(floor) = tile.floor {
                entity
                .insert(TileEntity {
                    x,
                    y,
                    floor: tile.floor,
                    walls: tile.walls,
                })
                .insert_bundle(PbrBundle {
                    transform: Transform::from_xyz(x as f32, 0.0, y as f32), 
                    ..Default::default()
                });
               /* if let Some(material_def) = metadata.materials.get_mut(&floor) {
                   /* if material_def.handle == None {
                        let material = StandardMaterial {
                            base_color_texture:Some(asset_server.load(&material_def.base_color_texture)),
                            metallic:0.0,
                            reflectance:0.0,
                            perceptual_roughness:1.0,
                            ..Default::default()
                        };
                        material_def.handle = Some(materials.add(material));
                    }
                    entity.insert_bundle(PbrBundle {
                        mesh:meshes.floor.clone(),
                        material:material_def.handle.clone().unwrap(),
                        transform:Transform::from_xyz(x as f32 + 0.5, 0.0, y as f32 + 0.5),
                        ..Default::default()
                    });*/


                    commands.spawn()
                }*/
            }
        }
    }
}

fn update_tilemap_entities(meshes:Res<Meshes>, mut materials: ResMut<Assets<StandardMaterial>>,_asset_server:Res<AssetServer>, mut commands:Commands, tilemap:Res<Tilemap>, tiles:Query<(Entity, &TileEntity, &Children)>, mut metadata:ResMut<Metadata>) {
    if tilemap.is_changed() == false {
        return;
    }

    fn get_material(id:Option<Id>, metadata:&mut Metadata) -> Handle<StandardMaterial> {
        Handle::default()
    }

    tiles.for_each(|(entity, tile, children)| {
        if let Some(tile2) = tilemap.tiles.get((tile.x, tile.y)) {
            if Some(entity) != tile2.entity {
                commands.entity(entity).despawn_recursive();
            } else {
                if let (Some(floor), Some(walls)) = (children.get(0), children.get(1)) {
                    let mut floor = commands.entity(*floor);
                    floor.insert_bundle(PbrBundle {
                        mesh:meshes.floor.clone(),
                        visibility:Visibility { is_visible: tile2.floor != None },
                        material:get_material(tile.floor, &mut metadata),
                        transform:Transform::from_xyz( 0.5, 0.0, 0.5),
                        ..Default::default()
                    });

                    let mut walls = commands.entity(*walls);
                    walls.insert_bundle(PbrBundle {
                        mesh:meshes.walls.clone(),
                        material:get_material(tile.walls, &mut metadata),
                        visibility:Visibility { is_visible: tile2.walls != None },
                        transform:Transform::from_xyz( 0.5, 0.0, 0.5),
                        ..Default::default()
                    });
                }
            }
        }
    });
}

pub struct TilemapPlugin;

pub struct Meshes {
    pub floor:Handle<Mesh>,
    pub walls:Handle<Mesh>
}

pub fn setup(mut commands:Commands, mut meshes:ResMut<Assets<Mesh>>) {
    let floor = Mesh::from(Plane { size: 1.0 });
    let walls = Mesh::from(Cube { size: 1.0});
    let meshes = Meshes {
        floor:meshes.add(floor),
        walls:meshes.add(walls)
    };
    commands.insert_resource(meshes);
}

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Tilemap::default());
        app.add_startup_system(setup);
        app.add_system_to_stage(CoreStage::Update, update_tilemap_entities.after(tilemap_changed));
        app.add_system_to_stage(CoreStage::PreUpdate, tilemap_changed);
    }
}