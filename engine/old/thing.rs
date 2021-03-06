use generational_arena::Index;
use macroquad::prelude::{Vec2, IVec2, Vec3};
use parry2d::bounding_volume::AABB;
use rune::Any;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default, Any)]
#[serde(default)]
pub struct Thing {
    #[rune(get, set)]
    pub pickup:bool,
    #[rune(get, set)]
    pub delete:bool,
    #[rune(get, set)]
    pub atlas: u32,
    #[rune(get, set)]
    pub atlas_index: u16,
    pub pos: Vec2,
    pub vel: Vec2,
    
    #[rune(get, set)]
    pub player:bool,
    #[rune(get, set)]
    pub solid:bool,

    pub touched_thing:Option<Index>
}

impl Thing {
    pub fn get_center_tile(pos:Vec2) -> IVec2 {
        return IVec2::new(pos.x as i32, pos.y as i32);
    } 

    pub fn get_aabb(&self) -> AABB {
        AABB::from_points([&[self.pos.x - 0.5, self.pos.y - 0.5].into(), &[self.pos.x + 0.5, self.pos.y + 0.5].into()])
    }

    pub fn get_tiles(pos:Vec2) -> [IVec2;8] {
        let mut tiles = [IVec2::new(0, 0); 8];
        let center = Self::get_center_tile(pos);

        let r = [-1, 0, 1];
        let mut i = 0;
        for y in r {
            for x in r {
                if (x == 0  && y == 0) == false {
                    tiles[i] = IVec2::new(x + center.x, y + center.y);
                    i += 1;
                }
            }
        }
        
        return tiles;
    }

    pub fn get_tiles_in_front(pos:Vec2, dir:IVec2) -> [IVec2; 3] {
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
                tiles[i] = IVec2::new(center.x + x, center.y + dir.y);
                i += 1;
            }
        }

        return tiles;
    }

}
