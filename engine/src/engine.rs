pub use macroquad;
use macroquad::prelude::{load_string, load_texture, FilterMode};
pub use macroquad_tiled;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use crate::{Atlas, Tile, Tilemap, World};

pub struct Engine {
    pub texture_atlases: HashMap<u32, Atlas>,
    pub tile_prototypes: HashMap<u32, Tile>,
    pub world: World,
    pub script_engine: rhai::Engine,
    pub commands: Rc<RefCell<Vec<ScriptCommand>>>,
}

fn get_i64(map: &rhai::Map, key: &str) -> i64 {
    let v = map.get(key);
    if let Some(v) = v {
        if let Ok(v) = v.as_int() {
            return v;
        }
    }

    return i64::default();
}
fn get_bool(map: &rhai::Map, key: &str) -> bool {
    let v = map.get(key);
    if let Some(v) = v {
        if let Ok(v) = v.as_bool() {
            return v;
        }
    }

    return bool::default();
}

impl Default for Engine {
    fn default() -> Self {
        let mut engine = rhai::Engine::new();
        let commands: Rc<RefCell<Vec<ScriptCommand>>> = Rc::new(RefCell::new(Vec::new()));
        let cmd = commands.clone();
        engine.register_fn("load_map", move |path: &str| {
            cmd.as_ref()
                .borrow_mut()
                .push(ScriptCommand::LoadMap { path: path.into() });
        });

        let cmd = commands.clone();
        engine.register_fn("define_tile", move |id:i64, tile:rhai::Map| {
            let atlas = get_i64(&tile, "atlas");
            let atlas_index = get_i64(&tile, "atlas_index");
            let solid = get_bool(&tile, "solid");

            cmd.as_ref().borrow_mut().push(ScriptCommand::DefineTile {
                id: id as u32,
                tile: Tile {  solid, atlas:atlas as u32, atlas_index:atlas_index as u16 },
            });
        });

        let cmd = commands.clone();
        engine.register_fn(
            "define_atlas",
            move |id: i64, columns: i64, rows: i64, texture_path: String| {
                cmd.borrow_mut().push(ScriptCommand::DefineAtlas {
                    id: id as u32,
                    columns: columns as u16,
                    rows: rows as u16,
                    texture_path: texture_path,
                })
            },
        );

        Self {
            texture_atlases: HashMap::new(),
            tile_prototypes: HashMap::new(),
            world: Default::default(),
            script_engine: engine,
            commands,
        }
    }
}

pub enum ScriptCommand {
    LoadMap {
        path: String,
    },
    DefineTile {
        id:u32,
        tile: Tile,
    },
    DefineAtlas {
        id: u32,
        columns: u16,
        rows: u16,
        texture_path: String,
    },
}

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

        let mut world = World::default();
        world.tilemap = Tilemap::new(&map, self.tile_prototypes.clone());
        self.world = world;
        println!("{:?}", self.world.tilemap.width);
    }

    pub async fn process_commands(&mut self) {
        for cmd in self.commands.clone().as_ref().borrow_mut().drain(..) {
            match cmd {
                ScriptCommand::LoadMap { path } => {
                    self.load_map(&path).await;
                }
                ScriptCommand::DefineTile { id, tile } => {
                    self.tile_prototypes.insert(id, tile);
                }
                ScriptCommand::DefineAtlas {
                    id,
                    columns,
                    rows,
                    texture_path,
                } => {
                    let texture = load_texture(&texture_path).await.unwrap();
                    self.texture_atlases.insert(
                        id,
                        Atlas {
                            texture,
                            columns,
                            rows,
                        },
                    );
                }
            }
        }
    }

    pub async fn eval(&mut self, script: &str) {
        match self.script_engine.eval::<()>(script) {
            Ok(_) => {}
            Err(err) => {
                println!("error executing script: {}", err);
            }
        }

        self.process_commands().await;
    }
    pub async fn eval_file(&mut self, path: &str) {
        match self.script_engine.eval_file::<()>(path.into()) {
            Ok(_) => {}
            Err(err) => {
                println!("error executing script: {}", err);
            }
        }

        self.process_commands().await;
    }
}
