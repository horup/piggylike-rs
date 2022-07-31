use bevy::prelude::*;
use serde::{Serialize, Deserialize};
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Cam {
    pub zoom:f32
}