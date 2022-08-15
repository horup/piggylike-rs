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
    pub x:u32,
    pub y:u32
}

fn tilemap_changed(mut commands:Commands, meshes:Res<Meshes>, mut materials: ResMut<Assets<StandardMaterial>>,asset_server:Res<AssetServer>, mut tilemap:ResMut<Tilemap>, mut metadata:ResMut<Metadata>, _tile_sprites:Query<(Entity, &TileEntity)>) {
    if tilemap.is_changed() {
        for ((x, y), tile) in tilemap.tiles.indexed_iter_mut() {
            if tile.entity == None {
                tile.entity = Some(commands.spawn().id());
            }

            let mut entity = commands.entity(tile.entity.unwrap());
            if let Some(floor) = tile.floor {
                if let Some(material_def) = metadata.materials.get_mut(&floor) {
                    if material_def.handle == None {
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
                    });
                }
            }
        }
    }
}

fn update_tilemap_entities(_asset_server:Res<AssetServer>, _commands:Commands, tilemap:Res<Tilemap>, _tiles:Query<(Entity, &TileEntity, &mut Handle<Scene>)>, _metadata:Res<Metadata>) {
    if tilemap.is_changed() == false {
        return;
    }

   /* for (e, t, mut handle) in tiles.iter_mut() {
        let mut delete = false;
        if let Some(tile) = tilemap.get(t.x as i32, t.y as i32) {
            if let Some(tile_entity) = tile.entity {
                if e != tile_entity {
                    delete = true;
                } else if tile.floor != t.tile_def {
                    if let Some(tile_def) = metadata.tiles.get(&tile.floor) {
                        *handle = asset_server.load(&tile_def.mesh);
                    } 
                }
            } else {
                delete = true;
            }
        } else {
            delete = true;
        }

        if delete {
            commands.entity(e).despawn_recursive();
        }
    }*/
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
      //  app.add_system_to_stage(CoreStage::PostUpdate, update_tilemap_entities.after(tilemap_changed));
        app.add_system_to_stage(CoreStage::Update, tilemap_changed);
    }
}