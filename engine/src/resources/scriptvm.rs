
use std::{sync::{Arc, Mutex}, path::{Path, PathBuf}};

use bevy::{asset::{AssetServerSettings, FileAssetIo}, prelude::{Res, AssetServer, Commands, ResMut, Assets, World}, sprite::TextureAtlas, math::Vec2};
use rune::{Module, Sources, Source, Diagnostics, prepare, termcolor::{StandardStream, ColorChoice}, Vm, runtime::Object, Value};

use crate::{resources::{Metadata, Id, AtlasDef, TileDef, ThingDef}, tiled::load_map};

pub struct ScriptVm {
    pub vm:Arc<Mutex<Vm>>
}
unsafe impl Send for ScriptVm {}
unsafe impl Sync for ScriptVm {}


#[derive(Clone)]
pub enum APICommand {
    DefineAtlas((Id, Object)),
    DefineTile((Id, Object)),
    DefineThing((Id, Object)),
    LoadMap(String)
}

#[derive(rune::Any, Default)]
pub struct ExclusiveContext {
    commands:Vec<APICommand>,
}

impl ExclusiveContext {
    pub fn process(&mut self, world:&mut World) {
        for cmd in self.commands.drain(..) {
            match cmd {
                APICommand::DefineAtlas((id, atlas)) => {
                    let id = id.clone();
                    let columns = get_i64(atlas.get("columns"));
                    let rows = get_i64(atlas.get("rows"));
                    let width = get_f32(atlas.get("width"));
                    let height = get_f32(atlas.get("height"));
                    let texture_path = get_string(atlas.get("texture_path"));
        
                    let texture_handle = world.get_resource::<AssetServer>().unwrap().load(&texture_path);
                    let mut texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(width, height), columns as usize, rows as usize);
                    let texture_atlas_handle = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap().add(texture_atlas);
                    
                    world.get_resource_mut::<Metadata>().unwrap().atlases.insert(id, AtlasDef {
                        handle:texture_atlas_handle
                    });
                },
                APICommand::DefineTile((id, tile)) => {
                    let atlas = get_i64(tile.get("atlas")) as u64;
                    let atlas_index = get_i64(tile.get("atlas_index")) as u32;
                    let solid = get_bool(tile.get("solid"));
        
                    world.get_resource_mut::<Metadata>().unwrap().tiles.insert(id, TileDef {
                        atlas,
                        atlas_index,
                        solid: solid,
                    });
                },
                APICommand::LoadMap(map_path) => {
                    load_map(world, &map_path);
                },
                APICommand::DefineThing((id, thing)) => {
                    let atlas = get_i64(thing.get("atlas")) as Id;
                    let atlas_index = get_i64(thing.get("atlas_index")) as u32;
                    let solid = get_bool(thing.get("solid"));
                    let player = get_bool(thing.get("player"));
                    world.get_resource_mut::<Metadata>().unwrap().things.insert(id, ThingDef {
                        atlas,
                        atlas_index,
                        player,
                        solid,
                    });
                },
            }
        }
    }

    pub fn define_atlas(&mut self, id:Id, atlas:Object) {
        self.commands.push(APICommand::DefineAtlas((id, atlas)));
    }

    pub fn define_tile(&mut self, id:Id, tile:Object) {
        self.commands.push(APICommand::DefineTile((id, tile)));
    }

    pub fn define_thing(&mut self, id:Id, thing:Object) {
        self.commands.push(APICommand::DefineThing((id, thing)));
    }

    pub fn load_map(&mut self, map:String) {
        self.commands.push(APICommand::LoadMap(map));
    }

    pub fn register(module:&mut Module) {
        module.ty::<ExclusiveContext>();
        module.inst_fn("define_atlas", Self::define_atlas).unwrap();
        module.inst_fn("define_tile", Self::define_tile).unwrap();
        module.inst_fn("define_thing", Self::define_thing).unwrap();
        module.inst_fn("load_map", Self::load_map).unwrap();
    }
}

pub fn get_f32(value: Option<&Value>) -> f32 {
    if let Some(value) = value.to_owned() {
        if let Ok(value) = value.to_owned().into_float() {
            return value as f32;
        }
    }

    return 0.0;
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


//pub fn setup(mut commands:Commands, mut metadata:ResMut<Metadata>, asset_server:Res<AssetServer>, mut texture_atlases:ResMut<Assets<TextureAtlas>>, world: &mut World) {
pub fn setup(world: &mut World) {
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let asset_io = asset_server.asset_io().downcast_ref::<FileAssetIo>().unwrap();
    let assets_path = asset_io.root_path().clone();
    let mut script_path = assets_path.clone();
    script_path.push("scripts");
    script_path.push("main.rn");
   

    let mut module = Module::with_crate("engine");
    ExclusiveContext::register(&mut module);

    let mut context = rune_modules::default_context().unwrap();
    context.install(&module).unwrap();
    let mut runtime = Arc::new(context.runtime());

    let mut sources = Sources::new();
    let source = Source::from_path(script_path.as_path()).unwrap();
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
    
    let mut api = ExclusiveContext::default();
    vm.call(&["main"], (&mut api, )).unwrap();
    api.process(world,);

    //world.insert_resource(vm);
    world.insert_resource(ScriptVm {
        vm:Arc::new(Mutex::new(vm))
    });
}