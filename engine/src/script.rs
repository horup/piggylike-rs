pub use macroquad;
use macroquad::prelude::{load_string, load_texture, FilterMode, Texture2D, Vec2};
pub use macroquad_tiled;
use rune::compile::Named;
use rune::runtime::{Object, UnsafeToValue};
use rune::termcolor::{ColorChoice, StandardStream};
use rune::*;
use rune::{prepare, Diagnostics, Module, Source, Sources, Value, Vm};
use std::borrow::BorrowMut;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::{cell::RefCell, collections::HashMap};

use crate::{Atlas, Command, Commands, Engine, Sprite, Thing, Tile, Tilemap, World};


#[derive(Any)]
struct ScriptWorld {
    #[rune(get, set)]
    pub things: Vec<Thing>,
}

fn register_types(module:&mut Module) {
    module.ty::<Thing>();
    module.ty::<ScriptWorld>();
}

fn register_functions(module: &mut Module, commands: Commands) {
    let mut cmds = commands.clone();
    module
        .function(&["define_atlas"], move |id: i64, atlas: Object| {
            let id = id.clone();
            let columns = get_i64(atlas.get("columns"));
            let rows = get_i64(atlas.get("rows"));
            let texture_path = get_string(atlas.get("texture_path"));
            cmds.push(Command::DefineAtlas {
                id: id as u32,
                columns: columns as u16,
                rows: rows as u16,
                texture_path,
            });
        })
        .unwrap();

    let mut cmds = commands.clone();
    module
        .function(&["define_tile"], move |id: i64, tile: Object| {
            let id = id.clone() as u32;
            let atlas = get_i64(tile.get("atlas")) as u32;
            let atlas_index = get_i64(tile.get("atlas_index")) as u16;
            let solid = get_bool(tile.get("solid"));
            cmds.push(Command::DefineTile {
                id: id as u32,
                tile: Tile {
                    atlas_index,
                    atlas,
                    solid,
                },
            });
        })
        .unwrap();

    let mut cmds = commands.clone();
    module
        .function(&["define_thing"], move |id: i64, thing: Object| {
            let id = id.clone() as u32;
            let atlas = get_i64(thing.get("atlas")) as u32;
            let atlas_index = get_i64(thing.get("atlas_index")) as u16;
            let solid = get_bool(thing.get("solid"));
            let player = get_bool(thing.get("player"));
            cmds.push(Command::DefineThing {
                id: id,
                thing: Thing {
                    atlas,
                    atlas_index,
                    solid,
                    player,
                    ..Default::default()
                },
            });
        })
        .unwrap();

    let mut cmds = commands.clone();
    module
        .function(&["load_map"], move |path: &str| {
            cmds.push(Command::LoadMap {
                path: String::from(path),
            });
        })
        .unwrap();
}

pub fn vm_create(script_path: &Path, commands: Commands) -> Vm {
    let mut module = Module::with_crate("engine");

    register_types(&mut module);
    register_functions(&mut module, commands.clone());

    let mut context = rune_modules::default_context().unwrap();
    context.install(&module).unwrap();
    let mut runtime = Arc::new(context.runtime());

    let mut sources = Sources::new();
    let source = Source::from_path(script_path).unwrap();
    sources.insert(source);

    let mut diagnostics = Diagnostics::new();
    let unit = prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources).unwrap();
    }

    let mut vm = Vm::new(runtime, Arc::new(unit.unwrap()));

    vm.call(&["main"], ()).unwrap();
    return vm;
}

pub fn get_i64(value: Option<&Value>) -> i64 {
    if let Some(value) = value.to_owned() {
        if let Ok(value) = value.to_owned().into_integer() {
            return value;
        }
    }

    return 0;
}

pub fn get_bool(value: Option<&Value>) -> bool {
    if let Some(value) = value.to_owned() {
        if let Ok(value) = value.to_owned().into_bool() {
            return value;
        }
    }

    return false;
}

pub fn get_string(value: Option<&Value>) -> String {
    if let Some(value) = value {
        if let Value::StaticString(s) = value {
            return s.to_string();
        }
    }

    return String::default();
}


impl From<&World> for ScriptWorld {
    fn from(w: &World) -> Self {
        ScriptWorld {
            things: w.things.iter().map(|(i, t)| t.clone()).collect(),
        }
    }
}

impl Engine {
    pub fn vm_update(&mut self) {
        let vm = &mut self.vm;

        let mut world = ScriptWorld::from(&self.world);

        vm.call(&["update"], (&mut world,)).unwrap();
    }
}
