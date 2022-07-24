pub use macroquad;
use macroquad::prelude::{load_string, load_texture, FilterMode, Vec2};
pub use macroquad_tiled;
use std::borrow::BorrowMut;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use crate::{Atlas, Engine, Sprite, Thing, Tile, Tilemap, World, Command};

impl Engine {
    pub fn new_script_engine(commands: Rc<RefCell<Vec<Command>>>) -> rhai::Engine {
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

        let mut engine = rhai::Engine::new();
        let cmd = commands.clone();
        engine.register_fn("load_map", move |path: &str| {
            cmd.as_ref()
                .borrow_mut()
                .push(Command::LoadMap { path: path.into() });
        });

        let cmd = commands.clone();
        engine.register_fn("define_tile", move |id: i64, tile: rhai::Map| {
            let atlas = get_i64(&tile, "atlas");
            let atlas_index = get_i64(&tile, "atlas_index");
            let solid = get_bool(&tile, "solid");

            cmd.as_ref().borrow_mut().push(Command::DefineTile {
                id: id as u32,
                tile: Tile {
                    solid,
                    atlas: atlas as u32,
                    atlas_index: atlas_index as u16,
                },
            });
        });

        let cmd = commands.clone();
        engine.register_fn("define_thing", move |id: i64, thing: rhai::Map| {
            let atlas = get_i64(&thing, "atlas");
            let atlas_index = get_i64(&thing, "atlas_index");
            let player = get_bool(&thing, "player");
            let solid = get_bool(&thing, "solid");
            let pickup = get_bool(&thing, "pickup");
            cmd.as_ref().borrow_mut().push(Command::DefineThing {
                id: id as u32,
                thing: Thing {
                    atlas: atlas as u32,
                    player: player,
                    solid: solid,
                    pickup: pickup,
                    atlas_index: atlas_index as u16,
                    ..Default::default()
                },
            });
        });

        let cmd = commands.clone();
        engine.register_fn(
            "define_atlas",
            move |id: i64, columns: i64, rows: i64, texture_path: String| {
                cmd.as_ref().borrow_mut().push(Command::DefineAtlas {
                    id: id as u32,
                    columns: columns as u16,
                    rows: rows as u16,
                    texture_path: texture_path,
                })
            },
        );

        let cmd = commands.clone();
        engine.register_fn(
            "load_world",
            move |file_name:String| {
                cmd.as_ref().borrow_mut().push(Command::LoadWorld { file_name: file_name });
            },
        );

        engine
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
