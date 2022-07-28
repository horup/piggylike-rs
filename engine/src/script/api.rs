use std::{cell::UnsafeCell, path::PathBuf};

use bevy::{prelude::{Commands, AssetServer, Res, ResMut, Assets, World}, sprite::TextureAtlas, math::Vec2, asset::FileAssetIo};
use rune::{Module, runtime::Object, Value};
use tiled::Loader;

use crate::{metadata::{Metadata, Id, AtlasDef, TileDef}, map::load_map};

#[derive(Clone)]
pub enum APICommand {
    DefineAtlas((Id, Object)),
    DefineTile((Id, Object)),
    LoadMap(String)
}

#[derive(rune::Any, Default)]
pub struct API {
    commands:Vec<APICommand>,
}

impl API {
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
                    let atlas_index = get_i64(tile.get("atlas")) as u32;
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
            }
        }
    }

    pub fn define_atlas(&mut self, id:Id, atlas:Object) {
        self.commands.push(APICommand::DefineAtlas((id, atlas)));
    }

    pub fn define_tile(&mut self, id:Id, tile:Object) {
        self.commands.push(APICommand::DefineTile((id, tile)));
    }

    pub fn load_map(&mut self, map:String) {
        self.commands.push(APICommand::LoadMap(map));
    }

    pub fn register(module:&mut Module) {
        module.ty::<API>();
        module.inst_fn("define_atlas", Self::define_atlas).unwrap();
        module.inst_fn("define_tile", Self::define_tile).unwrap();
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