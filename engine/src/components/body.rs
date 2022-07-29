use bevy::{prelude::Component, math::{Vec3, IVec2}};
use serde::{Serialize, Deserialize};

#[derive(Default, Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Body {
    pub pos:Vec3,
    pub vel:Vec3,
    pub size:f32,
    pub solid:bool
}

impl Body {
    pub fn get_center_tile(pos:Vec3) -> IVec2 {
        return IVec2::new(pos.x as i32, pos.y as i32);
    } 
    pub fn get_tiles_in_front(pos:Vec3, dir:IVec2) -> [IVec2; 3] {
        let center = Self::get_center_tile(pos);
        let mut tiles = [IVec2::new(0, 0);3];
        let mut i = 0;
        if dir.x != 0 {
            // vertical
            for y in [-1, 0, 1] {
                tiles[i] = IVec2::new(center.x + dir.x, center.y + y);
                i += 1;
            }
        } else if dir.y != 0 {
            // horizontal
            for x in [-1, 0, 1] {
                tiles[i] = IVec2::new(center.x + x, center.y - dir.y);
                i += 1;
            }
        }

        return tiles;
    }
}