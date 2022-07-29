use bevy::prelude::*;

use crate::components::{Controller, Player};

pub fn input_system(keys: Res<Input<KeyCode>>, mut player_query:Query<(&Player, &mut Controller)>) {
     let mut wish_dir = Vec3::default();
    wish_dir.x;
    if keys.pressed(KeyCode::A) {wish_dir.x -= 1.0}
    if keys.pressed(KeyCode::D) {wish_dir.x += 1.0}
    if keys.pressed(KeyCode::W) {wish_dir.y += 1.0}
    if keys.pressed(KeyCode::S) {wish_dir.y -= 1.0}
    
    wish_dir = wish_dir.normalize_or_zero();
    player_query.for_each_mut(|(_, mut controller)|controller.wish_dir = wish_dir);

}