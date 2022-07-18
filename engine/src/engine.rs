pub use macroquad;
use macroquad::prelude::{load_string, load_texture, FilterMode};
pub use macroquad_tiled;
use std::borrow::BorrowMut;
use std::{cell::RefCell, collections::HashMap};
use std::rc::Rc;

use crate::{Snapshot, Tilemap, Tile};

pub struct Engine {
    pub tile_prototypes:HashMap<u32, Tile>,
    pub world:Snapshot,
    pub script_engine:rhai::Engine,
    pub commands: Rc<RefCell<Vec<ScriptCommand>>>
}

impl Default for Engine {
    fn default() -> Self {
        let mut engine = rhai::Engine::new();
        let commands:Rc<RefCell<Vec<ScriptCommand>>> = Rc::new(RefCell::new(Vec::new()));
        let cmd = commands.clone();
        engine.register_fn("load_map", move |path:&str| {
            cmd.as_ref().borrow_mut().push(ScriptCommand::LoadMap { path: path.into() });
        });

        let mut cmd = commands.clone();
        engine.register_fn("define_tile", move |index:i64, solid:bool| {
            cmd.as_ref().borrow_mut().push(ScriptCommand::DefineTile { tile: Tile {
                index:index as u32,
                solid
            } });
        });

        Self { 
            tile_prototypes:HashMap::new(),
            world: Default::default(),
            script_engine:engine,
            commands
        }
    }
}

pub enum ScriptCommand {
    LoadMap { path:String },
    DefineTile { tile:Tile }
}

impl Engine {
    pub async fn load_map(&mut self, map_path:&str) {
        let map_json = load_string(map_path).await.unwrap();
        let tiles_tileset_json = load_string("assets/tilesets/tiles.tsj").await.unwrap();
        let things_tileset_json = load_string("assets/tilesets/things.tsj").await.unwrap();

        let tiles_texture = load_texture("assets/textures/tiles.png").await.unwrap();
        tiles_texture.set_filter(FilterMode::Nearest);
        let things_texture = load_texture("assets/textures/things.png").await.unwrap();
        things_texture.set_filter(FilterMode::Nearest);

        let texture_map = [("../textures/tiles.png", tiles_texture.clone()), ("../textures/things.png", things_texture.clone())];
        let tileset_map = [("../tilesets/tiles.tsj", tiles_tileset_json.as_str()), ("../tilesets/things.tsj", things_tileset_json.as_str())];

        let map = macroquad_tiled::load_map(&map_json, &texture_map, &tileset_map).unwrap();
        //self.world.map = Some(map);

        let mut snapshot = Snapshot::default();
        snapshot.tilemap = Tilemap::new(&map, self.tile_prototypes.clone());
    }

    pub async fn process_commands(&mut self) {
        for cmd in self.commands.clone().as_ref().borrow_mut().drain(..) {
            match cmd {
                ScriptCommand::LoadMap { path } => {
                    self.load_map(&path).await;
                },
                ScriptCommand::DefineTile { tile } => {

                },
            }
        }
    }

    pub async fn eval(&mut self, script:&str) {
        match self.script_engine.eval::<()>(script) {
            Ok(_) => {

            },
            Err(err) => {
                println!("error executing script: {}", err);
            },
        }

        self.process_commands();
    }
    pub async fn eval_file(&mut self, path:&str) {
        match self.script_engine.eval_file::<()>(path.into()) {
            Ok(_) => {

            },
            Err(err) => {
                println!("error executing script: {}", err);
            },
        }

        self.process_commands();
    }
}
