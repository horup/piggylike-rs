use macroquad::prelude::*;

use crate::Engine;

impl Engine {
    pub fn get_scale(&self) -> f32 {
        let visible_tiles = self.world.camera.visible_tiles;
        let scale = (screen_width() / visible_tiles).floor();
        return scale;
    }

    pub fn set_world_camera(&mut self) {
        let camera = &self.world.camera;
        let offset = camera.pos;
        let visible_tiles = camera.visible_tiles;
        let scale = self.get_scale();
        let offset = Vec2::new((offset.x * scale).floor() / scale, (offset.y * scale).floor() / scale);
        let mut r = Rect::new(camera.pos.x, camera.pos.y, screen_width() / scale, screen_height() / scale);
        r.x -= r.w / 2.0;
        r.y -= r.h / 2.0;
        let camera = Camera2D::from_display_rect(r);
        set_camera(&camera);
    }
    
    pub fn draw_tilemap(&self) {
        let tilemap = &self.world.tilemap;
        for layer in tilemap.layers.iter() {
            for y in 0..tilemap.height {
                for x in 0..tilemap.width {
                    if let Some(tile) = tilemap.get(0, x, y) {
                        let x = x as f32;
                        let y = y as f32;
                        if let Some(atlas) = self.atlases.get(&tile.atlas) {
                            atlas.draw(tile.atlas_index, x, y);
                        }
                    }
                }
            }
        }
    }

    pub fn draw_things(&self) {
        for (_, thing) in self.world.things.iter() {
            if let Some(atlas) = self.atlases.get(&thing.atlas) {
                atlas.draw(thing.atlas_index, (thing.pos.x * self.get_scale()).floor() / self.get_scale(), (thing.pos.y * self.get_scale()).floor() / self.get_scale());
            }
        }
    }

    pub fn draw_console(&self) {

    }
    
    pub fn draw(&mut self) {
        clear_background(WHITE);
        
        self.set_world_camera();
        self.draw_tilemap();
        self.draw_things();

        set_default_camera();
        self.draw_console();
    }
}