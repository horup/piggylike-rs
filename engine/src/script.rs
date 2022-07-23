pub use macroquad;
use macroquad::prelude::{load_string, load_texture, FilterMode, Vec2};
pub use macroquad_tiled;
use std::borrow::BorrowMut;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use crate::{Atlas, Engine, Sprite, Thing, Tile, Tilemap, World};

pub enum ScriptCommand {
    LoadMap {
        path: String,
    },
    DefineTile {
        id: u32,
        tile: Tile,
    },
    DefineAtlas {
        id: u32,
        columns: u16,
        rows: u16,
        texture_path: String,
    },
    DefineThing {
        id: u32,
        thing: Thing,
    },
    LoadWorld {
        file_name: String
    }
}

impl Engine {
    pub fn new_script_engine(commands: Rc<RefCell<Vec<ScriptCommand>>>) -> rhai::Engine {
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
                .push(ScriptCommand::LoadMap { path: path.into() });
        });

        let cmd = commands.clone();
        engine.register_fn("define_tile", move |id: i64, tile: rhai::Map| {
            let atlas = get_i64(&tile, "atlas");
            let atlas_index = get_i64(&tile, "atlas_index");
            let solid = get_bool(&tile, "solid");

            cmd.as_ref().borrow_mut().push(ScriptCommand::DefineTile {
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
            cmd.as_ref().borrow_mut().push(ScriptCommand::DefineThing {
                id: id as u32,
                thing: Thing {
                    atlas: atlas as u32,
                    player: player,
                    atlas_index: atlas_index as u16,
                    ..Default::default()
                },
            });
        });

        let cmd = commands.clone();
        engine.register_fn(
            "define_atlas",
            move |id: i64, columns: i64, rows: i64, texture_path: String| {
                cmd.as_ref().borrow_mut().push(ScriptCommand::DefineAtlas {
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
                cmd.as_ref().borrow_mut().push(ScriptCommand::LoadWorld { file_name: file_name });
            },
        );

        engine
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
                    texture.set_filter(FilterMode::Nearest);
                    self.atlases.insert(
                        id,
                        Atlas {
                            texture,
                            columns,
                            rows,
                        },
                    );
                }
                ScriptCommand::DefineThing { id, thing } => {
                    self.thing_prototypes.insert(id, thing);
                }
                ScriptCommand::LoadWorld { file_name } => self.load_world(&file_name),
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
