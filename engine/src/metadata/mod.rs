mod defs;
pub use defs::*;

use bevy::{prelude::*, utils::HashMap};

pub type Id = u64;

#[derive(Default, Clone)]
pub struct Metadata {
    pub atlases:HashMap<Id, AtlasDef>,
    pub tiles:HashMap<Id, TileDef>,
    pub things:HashMap<Id, ThingDef>
}


pub struct MetadataPlugin;

impl Plugin for MetadataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Metadata::default());
    }
}