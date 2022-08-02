use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, asset::FileAssetIo, render::texture::ImageSettings};

use crate::{systems, resources::ResourcesPlugin, events::EventsPlugin};
use systems::*;


pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    
    fn build(&self, app: &mut App) {

        app
        .insert_resource(Msaa {samples: 1})
        .add_plugin(ResourcesPlugin)
        .add_plugin(SystemsPlugin)
        .add_plugin(EventsPlugin)
        ;
    }
}

