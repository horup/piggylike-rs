use bevy::prelude::*;

use super::Snapshot;

pub struct History {
    pub interval_sec: f32,
    pub timer_sec: f32,
    pub history: Vec<Snapshot>,
}

impl Default for History {
    fn default() -> Self {
        Self {
            interval_sec: 0.1,
            timer_sec: 0.0,
            history: Default::default(),
        }
    }
}
