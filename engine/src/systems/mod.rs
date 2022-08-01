use bevy::prelude::*;

mod camera;
pub use camera::*;

mod controller;
pub use controller::*;

mod input;
pub use input::*;

mod physics;
pub use physics::*;

mod interpolation;
pub use interpolation::*;

mod render;
pub use render::*;

mod snapshot;
pub use snapshot::*;

mod spawn;
pub use spawn::*;

mod history;
pub use history::*;


pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // startup

        // preupdate
        app.add_system_to_stage(CoreStage::PreUpdate, history_system.exclusive_system().at_start());

        app.add_system_to_stage(CoreStage::PreUpdate, spawn_camera_system);
        app.add_system_to_stage(CoreStage::PreUpdate, spawn_tilemap_system);
        app.add_system_to_stage(CoreStage::PreUpdate, spawn_things_system);

        // update
        app.add_system(camera_system.after(interpolation_system).before(render_system));
        app.add_system(controller_system);
        app.add_system(input_system.before(controller_system));
        app.add_system(physics_system.after(controller_system));
        app.add_system(interpolation_system.after(physics_system));
        app.add_system(render_system);

        // post update
        app.add_system_to_stage(CoreStage::PostUpdate, snapshot_system.exclusive_system());
    }
}