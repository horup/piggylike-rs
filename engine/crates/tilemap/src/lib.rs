use bevy::{prelude::{*, shape::{Cube, Plane}}, utils::HashMap};

mod create_mesh;
use create_mesh::*;
mod shape;
use ndarray::{Array2, s, ArrayView2, Ix1};
pub use shape::*;
mod quad;
pub use quad::*;


use bevy::{prelude::Entity};
use serde::{Serialize, Deserialize};

use metadata::{Id, Metadata};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, Component, PartialEq)]
pub struct Tile {
    pub floor:Id,
    pub walls:Id,
    pub cealing:Id,
    pub top:f32,
    pub bottom:f32,
}

#[derive(Component, Default)]
pub struct TilemapMeshes {
    //pub entities:Vec<Entity>,
    pub chunks:HashMap<(usize, usize), Entity>
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tilemap {
    pub tiles:Array2<Tile>,
    pub top:f32,
    pub bottom:f32
}


impl Tilemap {
    pub fn new(width:usize, height:usize) -> Self {
        Self { tiles: Array2::default((width, height)), top:0.0, bottom:0.0 }
    }

    pub fn width(&self) -> usize {
        self.tiles.dim().0
    }

    pub fn height(&self) -> usize {
        self.tiles.dim().1
    }

    pub fn min_bottom(&self) -> f32 {
        let mut min_bottom = 0.0;
        self.tiles.for_each(|t| {
            if t.bottom < min_bottom {
                min_bottom = t.bottom;
            }
        });

        return min_bottom;
    }

    pub fn max_top(&self) -> f32 {
        let mut max_top = 0.0;
        self.tiles.for_each(|t| {
            if t.top > max_top {
                max_top = t.top;
            }
        });

        return max_top;
    }
}

#[derive(Component)]
struct Top;


#[derive(Component)]
struct Bottom;

#[derive(Component)]
struct Chunk {
    pub tiles:Array2<Tile>
}

impl Chunk {
    pub fn changed(&self, tiles:&ArrayView2<Tile>) -> bool {
        if self.tiles.dim().0 != tiles.dim().0 || self.tiles.dim().1 != tiles.dim().1 {
            return true;
        }

        for (p, tile) in self.tiles.indexed_iter() {
           
            if tile.ne(&tiles[p]) {
                return true;
            }
        }

        return false;
    }
}


fn collect_material_ids(tiles:&ArrayView2<Tile>) -> Vec<Id> {
    let mut materials = HashMap::new();
    for tile in tiles.iter() {
        materials.insert(tile.floor.clone(), true);
        materials.insert(tile.walls.clone(), true);
        materials.insert(tile.cealing.clone(), true);
    }

    let keys = materials.into_iter().map(|(id, _)| id).collect();
    return keys;
}

fn tilemap_changed(mut commands:Commands, mut materials:ResMut<Assets<StandardMaterial>>, mut meshes:ResMut<Assets<Mesh>>, asset_server:Res<AssetServer>, mut tilemaps:Query<(&mut Tilemap, &mut TilemapMeshes)>, mut metadata:ResMut<Metadata>, chunks:Query<&Chunk>) {
    for (mut tilemap, mut tile_meshes) in tilemaps.iter_mut() {
        if tilemap.is_changed() { 
            if tilemap.width() == 0 || tilemap.height() == 0 {
                return;
            }


            let chunk_size = 32.min(tilemap.width().min(tilemap.height()));
            let min_bottom = tilemap.min_bottom();
            let max_top = tilemap.max_top();
            let global_update = min_bottom != tilemap.bottom || max_top != tilemap.top;
            for cy in 0..tilemap.width() / chunk_size {
                for cx in 0..tilemap.height() / chunk_size {
                    let x = cx * chunk_size;
                    let y = cy * chunk_size;
                    let tiles = &tilemap.tiles.slice(s![x.. x + chunk_size, y..y + chunk_size]);
                    let mut update = false;
                    if let Some(chunk) = tile_meshes.chunks.get(&(cx, cy)) {
                        if let Ok(chunk) = chunks.get(*chunk) {
                            if chunk.changed(tiles) {
                                update = true;
                            }
                        }
                    } else {
                        update = true
                    }

                    update |= global_update;

                    if update {
                        if let Some(e) = tile_meshes.chunks.get(&(cx,cy)) {
                            commands.entity(*e).despawn_recursive();
                        }

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
                        let material_ids = collect_material_ids(tiles);

                        let parent = commands.spawn_bundle(PbrBundle {
                            transform:Transform::from_xyz(x as f32, 0.0, y as f32),
                            ..Default::default()
                        }).insert(Chunk {
                            tiles:tiles.to_owned()
                        }).id();

                        for material in material_ids.into_iter() {
                            let mesh = create_mesh(tiles, material, min_bottom, max_top);
                            let e = commands.spawn_bundle(PbrBundle {
                                mesh:meshes.add(mesh),
                                material:get_material(material),
                                ..Default::default()
                            }).id();
                            commands.entity(parent).add_child(e);
                        }

                        tile_meshes.chunks.insert((cx,cy), parent);

                    }
                }
            }

            tilemap.bottom = min_bottom;
            tilemap.top = max_top;
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