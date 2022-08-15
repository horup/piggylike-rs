use bevy::{prelude::{Plugin, Handle, StandardMaterial}};
use std::collections::HashMap;
pub type Id = u64;

#[derive(Default, Clone)]
pub struct TileDef {
    pub name:String,
    pub solid:bool,
    pub mesh:String
}

#[derive(Default, Clone)]
pub struct ThingDef {
    pub player:bool,
    pub solid:bool,
    pub mesh:String
}

#[derive(Default, Clone)]
pub struct MaterialDef {
    pub name:String,
    pub base_color_texture:String,
    pub handle:Option<Handle<StandardMaterial>>
}


#[derive(Default, Clone)]
pub struct Metadata {
    pub tiles:HashMap<Id, TileDef>,
    pub things:HashMap<Id, ThingDef>,
    pub materials:HashMap<Id, MaterialDef>
}

pub struct MetadataPlugin;

impl Plugin for MetadataPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Metadata::default());
    }
}