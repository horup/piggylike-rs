use bevy::prelude::*;

mod shape;
pub use shape::*;


use bevy::{math::{Vec3, IVec3}, prelude::Entity};
use serde::{Serialize, Deserialize};

use metadata::{Id, Metadata};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct Tile {
    pub solid:bool,
    pub tile_def:Id,
    #[serde(skip_serializing)]
    pub entity:Option<Entity>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tilemap {
    pub width:u32,
    pub height:u32,
    pub tiles:Vec<Option<Tile>>
}


impl Tilemap {
    pub fn new(width:u32, height:u32) -> Self {
        let mut vec = Vec::with_capacity(width as usize * height as usize);
        for _ in 0..vec.capacity() {
            vec.push(None);
        }
        
        Self { width, height, tiles: vec }
    }

    pub fn get_pos(&self, pos:Vec3) -> &Option<Tile> {
        let pos:IVec3 = pos.as_ivec3();
        return self.get(pos.x, pos.y);
    }

    pub fn get(&self, x:i32, y:i32) -> &Option<Tile> {
        if x < 0 || x > self.width as i32 || y < 0 || y > self.height as i32 {
            return &None;
        }

        let index = y * self.width as i32 + x;
        let index = index as usize;
        
        if let Some(tile) = self.tiles.get(index) {
            return tile;
        }

        return &None; 
    }

    pub fn get_mut(&mut self, x:i32, y:i32) -> Option<&mut Tile> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return None;
        }

        let index = y * self.width as i32 + x;
        let index = index as usize;
        
        if let Some(tile) = self.tiles.get_mut(index) {
            if let Some(tile) = tile {
                return Some(tile);
            }
        }

        return None; 
    }

    pub fn set(&mut self, x:i32, y:i32, tile:Option<Tile>) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }

        let index = y * self.width as i32 + x;
        let index = index as usize;
        self.tiles[index] = tile;
    }
}

#[derive(Component)]
struct TileEntity {
    pub x:u32,
    pub y:u32,
    pub tile_def:Id
}

fn tilemap_changed(mut commands:Commands, _meshes:ResMut<Assets<Mesh>>, _materials: ResMut<Assets<StandardMaterial>>,asset_server:Res<AssetServer>, mut tilemap:ResMut<Tilemap>, metadata:Res<Metadata>, _tile_sprites:Query<(Entity, &TileEntity)>) {
    if tilemap.is_changed() {
        for y in 0..tilemap.height {
            for x in 0..tilemap.width {
                if let Some(tile) = tilemap.get_mut(x as i32, y as i32) {
                    if let Some(tile_def) = metadata.tiles.get(&tile.tile_def) {
                        if tile_def.mesh.len() == 0 || tile.entity != None {
                            continue;
                        }
                        let mut e = commands.spawn();
                        e.insert_bundle(SceneBundle {
                            scene:asset_server.load(&tile_def.mesh),
                            transform: Transform::from_xyz(x as f32 + 0.5, 0.0, y as f32 + 0.5),
                            ..Default::default()
                        }).insert(TileEntity{x:x, y:y, tile_def:tile.tile_def});

                        tile.entity = Some(e.id());
                        
                    }
                }
            }
        }
    }
}

fn update_tilemap_entities(asset_server:Res<AssetServer>, mut commands:Commands, tilemap:Res<Tilemap>, mut tiles:Query<(Entity, &TileEntity, &mut Handle<Scene>)>, metadata:Res<Metadata>) {
    if tilemap.is_changed() == false {
        return;
    }

    for (e, t, mut handle) in tiles.iter_mut() {
        let mut delete = false;
        if let Some(tile) = tilemap.get(t.x as i32, t.y as i32) {
            if let Some(tile_entity) = tile.entity {
                if e != tile_entity {
                    delete = true;
                } else if tile.tile_def != t.tile_def {
                    if let Some(tile_def) = metadata.tiles.get(&tile.tile_def) {
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
    }
}

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Tilemap::default());
        app.add_system_to_stage(CoreStage::PostUpdate, update_tilemap_entities.after(tilemap_changed));
        app.add_system_to_stage(CoreStage::PostUpdate, tilemap_changed);
    }
}