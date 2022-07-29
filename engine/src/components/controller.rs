use bevy::{prelude::Component, math::Vec3};
use serde::{Serialize, Deserialize};

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Controller {
    pub wish_dir:Vec3
}