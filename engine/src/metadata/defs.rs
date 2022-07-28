use bevy::{prelude::Handle, sprite::TextureAtlas};

use super::Id;

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
    pub index:u32,
    pub player:bool,
    pub solid:bool
}