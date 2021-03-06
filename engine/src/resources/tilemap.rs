use bevy::{math::{Vec3, IVec3}, prelude::Entity};
use serde::{Serialize, Deserialize};

use crate::resources::Id;

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
        for i in 0..vec.capacity() {
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
        if x < 0 || x > self.width as i32 || y < 0 || y > self.height as i32 {
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
        if x < 0 || x > self.width as i32 || y < 0 || y > self.height as i32 {
            println!("out of bounds");
            return;
        }

        let index = y * self.width as i32 + x;
        let index = index as usize;
        self.tiles[index] = tile;
    }
}