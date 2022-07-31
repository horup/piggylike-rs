use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, asset::FileAssetIo};

use crate::{script::ScriptPlugin, metadata::MetadataPlugin, systems, resources::ResourcesPlugin};
use systems::*;


pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_plugin(MetadataPlugin)
        .add_plugin(ScriptPlugin)
        .add_plugin(ResourcesPlugin)
        .add_system(camera_system.after(interpolation_system).before(render_system))
        .add_system(controller_system)
        .add_system(input_system.before(controller_system))
        .add_system(physics_system.after(controller_system))
        .add_system(interpolation_system.after(physics_system))
        .add_system(render_system)
        .add_system_to_stage(CoreStage::PostUpdate, snapshot_system.exclusive_system())
        .add_system_to_stage(CoreStage::PreUpdate, spawn_camera)
        .add_system_to_stage(CoreStage::PreUpdate, spawn_tilemap)
        ;
    }
}

