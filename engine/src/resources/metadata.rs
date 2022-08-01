use std::collections::HashMap;
use bevy::{prelude::Handle, sprite::TextureAtlas};

pub type Id = u64;

#[derive(Default, Clone)]
pub struct AtlasDef {
    pub handle:Handle<TextureAtlas>
}

#[derive(Default, Clone)]
pub struct TileDef {
    pub atlas:Id,
    pub atlas_index:u32,
    pub solid:bool
}

#[derive(Default, Clone)]
pub struct ThingDef {
    pub atlas:Id,
    pub atlas_index:u32,
    pub player:bool,
    pub solid:bool
}


#[derive(Default, Clone)]
pub struct Metadata {
    pub atlases:HashMap<Id, AtlasDef>,
    pub tiles:HashMap<Id, TileDef>,
    pub things:HashMap<Id, ThingDef>
}