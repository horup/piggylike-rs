use macroquad::prelude::*;

use crate::{Engine, World, Tilemap};

impl Engine {
    pub async fn load_map(&mut self, map_path: &str) {
        let map_json = load_string(map_path).await.unwrap();
        let tiles_tileset_json = load_string("assets/tilesets/tiles.tsj").await.unwrap();
        let things_tileset_json = load_string("assets/tilesets/things.tsj").await.unwrap();

        let tiles_texture = load_texture("assets/textures/tiles.png").await.unwrap();
        tiles_texture.set_filter(FilterMode::Nearest);
        let things_texture = load_texture("assets/textures/things.png").await.unwrap();
        things_texture.set_filter(FilterMode::Nearest);

        let texture_map = [
            ("../textures/tiles.png", tiles_texture.clone()),
            ("../textures/things.png", things_texture.clone()),
        ];
        let tileset_map = [
            ("../tilesets/tiles.tsj", tiles_tileset_json.as_str()),
            ("../tilesets/things.tsj", things_tileset_json.as_str()),
        ];

        let map = macroquad_tiled::load_map(&map_json, &texture_map, &tileset_map).unwrap();
        //self.world.map = Some(map);

        self.world = World::default();
        self.world.tilemap = Tilemap::new(&map, self.tile_prototypes.clone());

        // load things
        if let Some(layer) = map.layers.get("things") {
            for object in layer.objects.iter() {
                if let Some(gid) = object.gid {
                    let id = gid - 1; // hack assuming things is first tileset
                    if let Some(&thing_prototype) = self.thing_prototypes.get(&id) {
                        let mut thing = thing_prototype.clone();
                        thing.pos = Vec2::new(object.world_x / object.world_w + 0.5, object.world_y / object.world_h - 0.5);
                        self.world.things.insert_with(|index| {
                            //thing.id = index;
                            thing
                        });

                    } else {
                        self.warn(&format!("thing {} not defined", id));
                    }
                }
            }
        }
    }
}