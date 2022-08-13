use bevy::{prelude::{Plugin, ResMut, DetectChanges, Color, Commands, AmbientLight}};
use metadata::Id;
use ndarray::{Array2, IndexLonger};
use serde::{Serialize, Deserialize};
use tilemap::Tilemap;


#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
pub struct Tile {
    pub tile_def:Id
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Map {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub tiles: Array2<Option<Tile>>,
    pub ambient_light:Color,
    pub ambient_brightness:f32
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
            tiles: Array2::default((size, size)),
            ambient_light:Color::WHITE,
            ambient_brightness:1.0
        }
    }
}


fn map_changed(mut commands:Commands, mut map:ResMut<Map>, mut tilemap:ResMut<Tilemap>) {
    if map.is_changed() {
        if map.width != tilemap.width as usize || map.height != tilemap.height as usize {
            *tilemap = Tilemap::new(map.width as u32, map.height as u32);
        }

        for ((x,y), tile) in map.tiles.indexed_iter() {
            match tile {
                Some(tile) => {
                    if let Some(t) = tilemap.get_mut(x as i32, y as i32) {
                        t.tile_def = tile.tile_def;
                    } else {
                        tilemap.set(x as i32, y as i32, Some(tilemap::Tile {
                            tile_def:tile.tile_def,
                            ..Default::default()
                        }))
                    }
                },
                None => tilemap.set(x as i32, y as i32, None),
            }
        }

        commands.insert_resource(AmbientLight {
            color: map.ambient_light,
            brightness: map.ambient_brightness,
        })
    }
}


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Map::default());
        app.add_system(map_changed);
    }
}