use std::collections::VecDeque;

use bevy::prelude::*;

use super::Snapshot;

pub struct History {
    pub interval_sec: f32,
    pub timer_sec: f32,
    pub snapshots: VecDeque<Snapshot>,
    pub max_snapshots:usize
}

impl Default for History {
    fn default() -> Self {
        Self {
            interval_sec: 0.1,
            timer_sec: 0.0,
            snapshots: Default::default(),
            max_snapshots: 1000
        }
    }
}
