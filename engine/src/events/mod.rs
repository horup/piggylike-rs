use bevy::prelude::*;

mod touch;
pub use touch::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        //app.
        app.add_event::<TouchEvent>();
    }
}