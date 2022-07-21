#![allow(warnings, unused)]

mod world;
pub use world::*;

mod tilemap;
pub use tilemap::*;

mod atlas;
pub use atlas::*;

mod thing;
pub use thing::*;

mod sprite;
pub use sprite::*;

mod draw;
pub use draw::*;

mod script;
pub use script::*;

mod camera;
pub use camera::*;

mod input;
pub use input::*;

mod physics;
pub use physics::*;

pub use macroquad;
use macroquad::prelude::{load_string, load_texture, FilterMode, Vec2, get_frame_time};
pub use macroquad_tiled;
use std::borrow::BorrowMut;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

pub struct Engine {
    pub input:Input,
    pub thing_prototypes: HashMap<u32, Thing>,
    pub sprite_prototypes: HashMap<u32, Sprite>,
    pub atlases: HashMap<u32, Atlas>,
    pub tile_prototypes: HashMap<u32, Tile>,
    pub world: World,
    pub script_engine: rhai::Engine,
    pub commands: Rc<RefCell<Vec<ScriptCommand>>>,
}


impl Default for Engine {
    fn default() -> Self {
        let commands = Rc::new(RefCell::new(Vec::new()));
        Self {
            input:Input::default(),
            thing_prototypes: HashMap::new(),
            sprite_prototypes: HashMap::new(),
            atlases: HashMap::new(),
            tile_prototypes: HashMap::new(),
            world: Default::default(),
            script_engine: Self::new_script_engine(commands.clone()),
            commands:commands.clone(),
        }
    }
}


impl Engine {
    pub fn warn(&self, warn:&str) {
        println!("warning: {}", warn);
    }

    pub fn update(&mut self) {
        self.update_input();
        self.update_movement();
        self.draw();
    }

    pub fn get_delta_time(&self) -> f32 {
        let dt = get_frame_time();
        let max = 0.1;
        if dt < max {
            return dt;
        } else {
            return max;
        }
    }

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
                        thing.pos = Vec2::new(object.world_x / object.world_w, object.world_y / object.world_h);
                        self.world.things.insert_with(|index| {
                            thing.id = index;
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
