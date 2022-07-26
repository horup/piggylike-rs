pub use macroquad;
use macroquad::prelude::{load_string, load_texture, FilterMode, Vec2};
pub use macroquad_tiled;
use rune::termcolor::{StandardStream, ColorChoice};
use rune::{Vm, Sources, Source, prepare, Diagnostics, Module};
use std::borrow::BorrowMut;
use std::fs::read_to_string;
use std::path::{PathBuf, Path};
use std::rc::Rc;
use std::sync::Arc;
use std::{cell::RefCell, collections::HashMap};

use crate::{Atlas, Command, Engine, Sprite, Thing, Tile, Tilemap, World, Commands};

impl Engine {

    pub fn vm_create(script_path:&Path, commands: Commands) -> Vm {
        let mut module = Module::new();

        let mut cmds = commands.clone();
        module.function(&["test"], move ||{
            //cmds.push(Command::LoadWorld { file_name: "test.tmj".into() });

            cmds.push(Command::Execute { function: Box::new(|engine| {
                println!("hi from execute")
            })});
        }).unwrap();

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
}
