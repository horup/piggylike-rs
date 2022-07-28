use std::cell::UnsafeCell;

use bevy::{prelude::{Commands, AssetServer, Res, ResMut, Assets}, sprite::TextureAtlas, math::Vec2};
use rune::{Module, runtime::Object, Value};

use crate::metadata::{Metadata, Id, AtlasDef, TileDef};

#[derive(rune::Any, Default)]
pub struct API {
    define_atlases:Vec<(Id, Object)>,  
    define_tiles:Vec<(Id, Object)>
}

impl API {

    pub fn process(&mut self, mut metadata:&mut ResMut<Metadata>, commands:&mut Commands, asset_server:&Res<AssetServer>, texture_atlases:&mut ResMut<Assets<TextureAtlas>>) {
        self.process_atlases(metadata, commands, asset_server, texture_atlases);
        self.process_tiles(metadata);
    }

    pub fn process_atlases(&mut self, mut metadata:&mut ResMut<Metadata>, commands:&mut Commands, asset_server:&Res<AssetServer>, texture_atlases:&mut ResMut<Assets<TextureAtlas>>) {
        for (id, atlas) in self.define_atlases.drain(..) {
            let id = id.clone();
            let columns = get_i64(atlas.get("columns"));
            let rows = get_i64(atlas.get("rows"));
            let width = get_f32(atlas.get("width"));
            let height = get_f32(atlas.get("height"));
            let texture_path = get_string(atlas.get("texture_path"));

            let texture_handle = asset_server.load(&texture_path);
            let mut texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(width, height), columns as usize, rows as usize);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            
            metadata.atlases.insert(id, AtlasDef {
                handle:texture_atlas_handle
            });
        }
    }

    pub fn process_tiles(&mut self, mut metadata:&mut ResMut<Metadata>) {
        for (id, tile) in self.define_tiles.drain(..) {
            let atlas = get_i64(tile.get("atlas")) as u64;
            let atlas_index = get_i64(tile.get("atlas")) as u32;
            let solid = get_bool(tile.get("solid"));

            metadata.tiles.insert(id, TileDef {
                atlas,
                atlas_index,
                solid: solid,
            });
        }
    }

    pub fn define_atlas(&mut self, id:Id, atlas:Object) {
        self.define_atlases.push((id, atlas));
    }

    pub fn define_tile(&mut self, id:Id, tile:Object) {
        self.define_tiles.push((id, tile));
    }

    pub fn register(module:&mut Module) {
        module.ty::<API>();
        module.inst_fn("define_atlas", Self::define_atlas).unwrap();
        module.inst_fn("define_tile", Self::define_tile).unwrap();
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