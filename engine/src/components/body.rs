use bevy::{prelude::Component, math::Vec3};
use serde::{Serialize, Deserialize};

#[derive(Default, Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Body {
    pub position:Vec3,
    pub velocity:Vec3,
    pub size:f32,
    pub solid:bool
}