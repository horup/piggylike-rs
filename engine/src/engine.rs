pub use macroquad;
use macroquad::prelude::{load_string, load_texture, FilterMode};
pub use macroquad_tiled;
use std::cell::RefCell;
use std::rc::Rc;

use crate::World;


pub struct Engine {
    pub world:World,
    pub script_engine:rhai::Engine,
    pub commands:Rc<RefCell<Vec<ScriptCommand>>>
}

impl Default for Engine {
    fn default() -> Self {
        let mut engine = rhai::Engine::new();
        let commands:Rc<RefCell<Vec<ScriptCommand>>> = Rc::new(RefCell::new(Vec::new()));
        let cmd = commands.clone();
        
        engine.register_fn("load_map", move |path:&str| {
            cmd.borrow_mut().push(ScriptCommand::LoadMap { path: path.into() });
        });


        Self { 
            world: Default::default(),
            script_engine:engine,
            commands:commands
        }
    }
}

pub enum ScriptCommand {
    LoadMap { path:String }
}

impl Engine {
    pub async fn load_map(&mut self, map_path:&str) {
        let map_json = load_string(map_path).await.unwrap();
        let tiles_tileset_json = load_string("assets/tilesets/tiles.tsj").await.unwrap();
        let entities_tileset_json = load_string("assets/tilesets/entities.tsj").await.unwrap();

        let tiles_texture = load_texture("assets/textures/tiles.png").await.unwrap();
        tiles_texture.set_filter(FilterMode::Nearest);
        let entities_texture = load_texture("assets/textures/entities.png").await.unwrap();
        entities_texture.set_filter(FilterMode::Nearest);

        let texture_map = [("../textures/tiles.png", tiles_texture.clone()), ("../textures/entities.png", entities_texture.clone())];
        let tileset_map = [("../tilesets/tiles.tsj", tiles_tileset_json.as_str()), ("../tilesets/entities.tsj", entities_tileset_json.as_str())];

        let map = macroquad_tiled::load_map(&map_json, &texture_map, &tileset_map).unwrap();
        self.world.map = Some(map);
    }

    pub async fn eval(&mut self, script:&str) {
        match self.script_engine.eval::<()>(script) {
            Ok(_) => {

            },
            Err(err) => {
                println!("error executing script: {}", err);
            },
        }

        for cmd in self.commands.clone().borrow_mut().drain(..) {
            match cmd {
                ScriptCommand::LoadMap { path } => {
                    self.load_map(&path).await;
                },
            }
        }
    }
    pub async fn eval_file(&mut self, path:&str) {
        match self.script_engine.eval_file::<()>(path.into()) {
            Ok(_) => {

            },
            Err(err) => {
                println!("error executing script: {}", err);
            },
        }

        for cmd in self.commands.clone().borrow_mut().drain(..) {
            match cmd {
                ScriptCommand::LoadMap { path } => {
                    self.load_map(&path).await;
                },
            }
        }
        
    }
}
