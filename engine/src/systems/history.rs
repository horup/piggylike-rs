use bevy::prelude::*;

use crate::resources::{History, Snapshot};

pub fn history_system(world:&mut World) {
    if world.get_resource::<Input<KeyCode>>().unwrap().pressed(KeyCode::F1) {
        if let Some(mut history) = world.get_resource_mut::<History>() {
            if let Some(snapshot) = history.snapshots.pop_front() {
                world.clear_entities();
                snapshot.restore(world);
            }
        }
    };
}