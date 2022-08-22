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
            
            for (handle, mesh) in meshes.iter_mut() {
                let vs = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();
                match vs {
                    bevy::render::mesh::VertexAttributeValues::Float32x3(v) => {
                        dbg!(v.len());
                    },
                    _ => {}
                }
            }
        }
    }

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

    commands.spawn().insert(Tilemap::default()).insert(TilemapMeshes::default());
}


impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
       
        app.add_startup_system(setup);
      //  app.add_system_to_stage(CoreStage::Update, update_tilemap_entities.after(tilemap_changed));
        app.add_system_to_stage(CoreStage::PreUpdate, tilemap_changed);
    }
}