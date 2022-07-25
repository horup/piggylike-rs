pub use macroquad;
use macroquad::prelude::{load_string, load_texture, FilterMode, Vec2};
pub use macroquad_tiled;
use rhai::module_resolvers::FileModuleResolver;
use rhai::{Module, Scope, Dynamic};
use std::borrow::BorrowMut;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use crate::{Atlas, Command, Engine, Sprite, Thing, Tile, Tilemap, World};

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
        engine.set_module_resolver(FileModuleResolver::new_with_path(PathBuf::from("assets/scripts")));
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
        engine.register_fn("load_world", move |file_name: String| {
            cmd.as_ref().borrow_mut().push(Command::LoadWorld {
                file_name: file_name,
            });
        });

        engine
    }

    pub async fn update_script(&mut self) {
        //let ast = self.script_engine.compile("").unwrap();
        //let r:() = self.script_engine.call_fn(&mut self.global_scope, &rhai::AST::default(), "test", ()).unwrap();
        //scope.push("test",
        //let ast = self.script_engine.compile("fn abc(){test();}").unwrap();
        //let r:() = self.script_engine.call_fn(&mut Scope::new(), &ast, "abc", ()).unwrap();

        let r = self
            .script_engine
            .call_fn_raw(
                &mut self.global_scope,
                &self.script,
                false,
                true,
                "test",
                None,
                [],
            )
            .unwrap();

    }

    pub async fn register_script_file(&mut self, path: &str) {
        let ast = self.script_engine.compile_file_with_scope(&self.global_scope, path.into()).unwrap();
        self.script = ast;
        let res:() = self.script_engine.eval_ast_with_scope(&mut self.global_scope, &self.script).unwrap();
    }

}
