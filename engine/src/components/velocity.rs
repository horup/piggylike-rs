use bevy::{prelude::Component, math::Vec3};
use serde::{Serialize, Deserialize};

#[derive(Component, Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Velocity {
    pub velocity:Vec3
}