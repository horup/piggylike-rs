use bevy::prelude::*;

use crate::components::Controller;

pub fn controller_system(mut query:Query<(&Controller, &mut Transform)>) {
    query.for_each_mut(|(controller, mut transform)| {
        transform.translation += controller.wish_dir;
    });
}