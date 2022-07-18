use std::collections::HashMap;

use macroquad_tiled::Map;

#[derive(Default, Clone)]
pub struct Layer {
    pub tiles:Vec<Option<Tile>>
}

#[derive(Default, Clone)]
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
        for (map_layer_name, map_layer) in &map.layers {
            let mut tiles = Vec::new();
            for t in &map_layer.data {
                if let Some(t) = t {
                    println!("{:?}", map.raw_tiled_map.tilesets);                    
                    tiles.push(Some(Tile {
                        index:t.id,
                        solid:false
                    }));
                } else {
                    tiles.push(None);
                }
            }

            for o in &map_layer.objects {
                
            }

            let mut layer = Layer {
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
}