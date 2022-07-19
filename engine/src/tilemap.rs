use std::collections::HashMap;

use macroquad_tiled::Map;

#[derive(Default, Clone)]
pub struct Layer {
    pub tiles:Vec<Option<Tile>>
}

#[derive(Default, Clone, Copy)]
pub struct Tile {
    pub index:u32,
    pub solid:bool
}

#[derive(Clone, Default)]
pub struct Tilemap {
    pub layers:Vec<Layer>,
    pub width:u32,
    pub height:u32
}

impl Tilemap {
    pub fn new(map:&Map, tile_prototypes:HashMap<u32, Tile>) -> Self {
        let mut layers = Vec::new();
        for (_, map_layer) in &map.layers {
            let mut tiles = Vec::new();
            for t in &map_layer.data {
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

            for o in &map_layer.objects {
                
            }

            let layer = Layer {
                tiles
            };

            layers.push(layer);
        }


        Self {
            width:map.raw_tiled_map.width,
            height:map.raw_tiled_map.height,
            layers
        }
    }

    pub fn get(&self, layer:usize, x:u32, y:u32) -> Option<Tile> {
        let index = y * self.width + x;
        if let Some(layer) = self.layers.get(layer) {
            if let Some(tile) = layer.tiles.get(index as usize) {
                return *tile;
            }
        }

        return None; 
    }
}