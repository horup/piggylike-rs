use std::collections::HashMap;

use macroquad_tiled::Map;
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub tiles:Vec<Option<Tile>>
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Tile {
    pub atlas_index:u16,
    pub atlas:u32,
    pub solid:bool
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Tilemap {
    pub layers:Vec<Layer>,
    pub width:i32,
    pub height:i32
}

impl Tilemap {
    pub fn new(map:&Map, tile_prototypes:HashMap<u32, Tile>) -> Self {
        let mut layers = Vec::new();

        if let Some(map_tiles) = map.layers.get("tiles") {
            let mut tiles = Vec::new();
            for t in map_tiles.data.iter() {
                if let Some(t) = t {
                    if let Some(t) = tile_prototypes.get(&t.id) {
                        tiles.push(Some(t.clone()));
                    } else {
                        println!("missing tile defintion for {}", t.id);
                    }
                } else {
                    tiles.push(None);
                }
            }

            let layer = Layer {
                tiles
            };

            layers.push(layer);
        }

        Self {
            width:map.raw_tiled_map.width as i32,
            height:map.raw_tiled_map.height as i32,
            layers
        }
    }

    pub fn get(&self, layer:usize, x:i32, y:i32) -> Option<Tile> {
        let index = y.abs() * self.width + x;
        if let Some(layer) = self.layers.get(layer) {
            if let Some(tile) = layer.tiles.get(index as usize) {
                return *tile;
            }
        }

        return None; 
    }
}