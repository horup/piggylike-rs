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

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Tilesprite;

pub fn spawn_tilemap_system(mut commands:Commands, _meshes:ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,asset_server:Res<AssetServer>, mut tilemap:ResMut<Tilemap>, metadata:Res<Metadata>, tile_sprites:Query<(Entity, &Tilesprite)>) {
    if tilemap.is_changed() {
        tile_sprites.for_each(|(e,_)| {
            let c = tilemap.tiles.iter().filter(|p| {
                if let Some(tile) = p {
                    if let Some(tile_entity) = tile.entity {
                        if tile_entity == e {
                            return true;
                        }
                    }
                }

                return false;
            }).count();

            if c == 0 {
                commands.entity(e).despawn_recursive();
            }
        });


        tilemap.tiles.iter_mut().for_each(|tile| {
            if let Some(tile) = tile {
                tile.entity = None;
            }
        });

        let _mat = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
        for y in 0..tilemap.height {
            for x in 0..tilemap.width {
                if let Some(tile) = tilemap.get_mut(x as i32, y as i32) {
                    if let Some(tile_def) = metadata.tiles.get(&tile.tile_def) {
                        if tile_def.mesh.len() == 0 {
                            continue;
                        }
                        let mut e = commands.spawn();
                        e.insert_bundle(SceneBundle {
                            scene:asset_server.load(&tile_def.mesh),
                            transform: Transform::from_xyz(x as f32 + 0.5, 0.0, y as f32 + 0.5),
                            ..Default::default()
                        }).insert(Tilesprite);

                        tile.entity = Some(e.id());
                    }
                }
            }
        }
    }
}

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Tilemap::default());
        app.add_system_to_stage(CoreStage::PostUpdate, spawn_tilemap_system);
    }
}