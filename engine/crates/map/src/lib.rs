use bevy::{prelude::Plugin};
use metadata::Id;
use ndarray::{Array2};
use serde::{Serialize, Deserialize};


#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Tile {
    pub tile_id:Id
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub tiles: Array2<Option<Tile>>
}

impl Map {
    pub fn save(&self, path:&str) {
        let json = serde_json::to_string(self).unwrap();
        std::fs::write(&path, json).unwrap();
    }

    pub fn load(path:&str) -> Option<Map> {
        if let Ok(json) = std::fs::read_to_string(path) {
            if let Ok(map) = serde_json::from_str::<Map>(&json) {
                return Some(map);
            }
        }
        
        None
    }
}

impl Default for Map {
    fn default() -> Self {
        let size = 16;
        Self {
            name: String::from("Untitled"),
            width: size,
            height: size,
            tiles: Array2::default((size, size))
        }
    }
}


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Map::default());
    }
}