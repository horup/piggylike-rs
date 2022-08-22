use bevy::{prelude::{*, shape::{Cube, Plane}}, utils::HashMap};

mod create_mesh;
use create_mesh::*;
mod shape;
use ndarray::Array2;
pub use shape::*;
mod quad;
pub use quad::*;


use bevy::{prelude::Entity};
use serde::{Serialize, Deserialize};

use metadata::{Id, Metadata};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, Component)]
pub struct Tile {
    pub floor:Id,
    pub walls:Id,
    pub cealing:Id,
    pub top:f32,
    pub bottom:f32,
    #[serde(skip_serializing)]
    pub entity:Option<Entity>
}

#[derive(Component, Default)]
pub struct TilemapMeshes {
    pub entities:Vec<Entity>
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tilemap {
    pub tiles:Array2<Tile>,
}


impl Tilemap {
    pub fn new(width:usize, height:usize) -> Self {
        Self { tiles: Array2::default((width, height)), }
    }

    pub fn width(&self) -> usize {
        self.tiles.dim().0
    }

    pub fn height(&self) -> usize {
        self.tiles.dim().1
    }
}

#[derive(Component)]
struct Top;


#[derive(Component)]
struct Bottom;

#[derive(Component)]
struct TileIndex {
    pub x:usize,
    pub y:usize
}


fn collect_material_ids(tiles:&Array2<Tile>) -> Vec<Id> {
    let mut materials = HashMap::new();
    for tile in tiles.iter() {
        materials.insert(tile.floor.clone(), true);
        materials.insert(tile.walls.clone(), true);
        materials.insert(tile.cealing.clone(), true);
    }

    let keys = materials.into_iter().map(|(id, _)| id).collect();
    return keys;
}

fn tilemap_changed(mut commands:Commands, mut materials:ResMut<Assets<StandardMaterial>>, mut meshes:ResMut<Assets<Mesh>>, asset_server:Res<AssetServer>, mut tilemaps:Query<(&mut Tilemap, &mut TilemapMeshes)>, mut metadata:ResMut<Metadata>) {
    for (tilemap, mut tile_meshes) in tilemaps.iter_mut() {
        if tilemap.is_changed() { 
            for mesh in tile_meshes.entities.iter() {
                commands.entity(*mesh).despawn_recursive();
            }
            tile_meshes.entities.clear();

            let mut get_material = |id| -> Handle<StandardMaterial> {
                if let Some(def) = metadata.materials.get_mut(&id) {
                    if def.handle == None {
                        let material = StandardMaterial {
                            base_color_texture:Some(asset_server.load(&def.base_color_texture)),
                            metallic:0.0,
                            reflectance:0.0,
                            perceptual_roughness:1.0,
                            ..Default::default()
                        };
    
                        def.handle = Some(materials.add(material));
                    }
    
                    return def.handle.clone().unwrap();
                }
        
                Handle::default()
            };
            let material_ids = collect_material_ids(&tilemap.tiles);

            for material in material_ids.into_iter() {
                let mesh = create_mesh(&tilemap.tiles, material);
                let e = commands.spawn_bundle(PbrBundle {
                    mesh:meshes.add(mesh),
                    material:get_material(material),
                    ..Default::default()
                });

                tile_meshes.entities.push(e.id());
            }




            /*let e = commands.spawn_bundle(PbrBundle {
                mesh:meshes.add(mesh),
                ..Default::default()
            });*/
            
        }
    }
    /*if tilemap.is_changed() {
        if tilemaps.iter().count() == 0 {
            commands.spawn_bundle(PbrBundle {
                ..Default::default()
            }).insert(tilemap.clone());
        }
        /*for ((x, y), tile) in tilemap.tiles.indexed_iter_mut() {
            if tile.entity == None {
                tile.entity = Some(commands.spawn().with_children(|f|{
                    f.spawn().insert(Top);
                    f.spawn().insert(Bottom);
                }).id());
            }

            commands.entity(tile.entity.unwrap())
            .insert(tile.clone())
            .insert(TileIndex {x, y})
            .insert_bundle(PbrBundle {
                transform: Transform::from_xyz(x as f32, 0.0, y as f32), 
                ..Default::default()
            });
        }*/
    }*/
}
/*
fn update_tilemap_entities(meshes:Res<Meshes>, mut materials: ResMut<Assets<StandardMaterial>>,asset_server:Res<AssetServer>, mut commands:Commands, tilemap:Res<Tilemap>, tiles:Query<(Entity, &Tile,&TileIndex, &Children)>, mut metadata:ResMut<Metadata>) {
    if tilemap.is_changed() == false {
        return;
    }

    let mut get_material = |id:Option<Id>| -> Handle<StandardMaterial> {
        if let Some(id) = id {
            if let Some(def) = metadata.materials.get_mut(&id) {
                if def.handle == None {
                    let material = StandardMaterial {
                        base_color_texture:Some(asset_server.load(&def.base_color_texture)),
                        metallic:0.0,
                        reflectance:0.0,
                        perceptual_roughness:1.0,
                        ..Default::default()
                    };

                    def.handle = Some(materials.add(material));
                }

                return def.handle.clone().unwrap();
            }
        }
    
        Handle::default()
    };

    tiles.for_each(|(entity, tile, index, children)| {
        let mut delete = false;
        if let Some(tile2) = tilemap.tiles.get((index.x, index.y)) {
            if Some(entity) != tile2.entity {
                delete = true;
            } else {
                if let (Some(floor), Some(walls)) = (children.get(0), children.get(1)) {
                    let mut bottom = commands.entity(*floor);
                    bottom.insert_bundle(PbrBundle {
                        mesh:meshes.walls.clone(),
                        material:get_material(Some(tile.walls)),
                        transform:Transform::from_xyz( 0.5, tile.top, 0.5),
                        ..Default::default()
                    });

                    dbg!(tile.top);


              /*      let y = if tile.walls == None {0.0} else {1.0};
                    floor.insert_bundle(PbrBundle {
                        mesh:meshes.floor.clone(),
                        visibility:Visibility { is_visible: tile2.floor != None },
                        material:get_material(tile.floor),
                        transform:Transform::from_xyz( 0.5, y, 0.5),
                        ..Default::default()
                    });

                    let mut walls = commands.entity(*walls);
                    walls.insert_bundle(PbrBundle {
                        mesh:meshes.walls.clone(),
                        material:get_material(tile.walls),
                        visibility:Visibility { is_visible: tile2.walls != None },
                        transform:Transform::from_xyz( 0.5, 0.5, 0.5),
                        ..Default::default()
                    });*/
                }
            }
        } else {
            delete = true;
        }

        if delete {
            commands.entity(entity).despawn_recursive();
        }
    });
}*/

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

    commands.spawn().insert(Tilemap::default()).insert(TilemapMeshes::default());
}


impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
       
        app.add_startup_system(setup);
      //  app.add_system_to_stage(CoreStage::Update, update_tilemap_entities.after(tilemap_changed));
        app.add_system_to_stage(CoreStage::PreUpdate, tilemap_changed);
    }
}