use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, asset::FileAssetIo, render::texture::ImageSettings};
use bevy_flycam::{PlayerPlugin, NoCameraPlayerPlugin};
use bevy_mod_raycast::DefaultRaycastingPlugin;

use crate::{systems, resources::ResourcesPlugin, events::EventsPlugin, components::MyRaycastSet};
use systems::*;


pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    
    fn build(&self, app: &mut App) {

        app
        .insert_resource(Msaa {samples: 1})
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(ResourcesPlugin)
        .add_plugin(SystemsPlugin)
        .add_plugin(EventsPlugin)
        .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
        ;
    }
}

