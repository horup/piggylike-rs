use std::collections::HashMap;
use bevy::{prelude::Handle, sprite::TextureAtlas, scene::Scene};

pub type Id = u64;

#[derive(Default, Clone)]
pub struct TileDef {
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
pub struct Metadata {
    pub tiles:HashMap<Id, TileDef>,
    pub things:HashMap<Id, ThingDef>,
}