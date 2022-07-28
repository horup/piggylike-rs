use macroquad::prelude::{Texture2D, draw_texture_ex, Color, DrawTextureParams, draw_texture, WHITE, Rect, Vec2};

pub struct Atlas {
    pub texture:Texture2D,
    pub columns:u16,
    pub rows:u16
}

impl Atlas {
    pub fn new(texture:Texture2D, columns:u16, rows:u16) -> Self {
        Self { texture, columns, rows }
    }

    pub fn draw(&self, index:u16, x:f32, y:f32) {
        let sw = self.texture.width() / self.columns as f32;
        let sh = self.texture.height() / self.rows as f32;
        let sx = index % self.columns as u16;
        let sy = index / self.columns as u16;
        let sx = sx as f32 / self.columns as f32 * self.texture.width();
        let sy = sy as f32 / self.rows as f32 * self.texture.height();

        let alpha_w = 0.1;
        let alpha_h = 0.1;
        let src = Rect::new(sx + alpha_w, sy + alpha_h, sw - alpha_w * 2.0, sh - alpha_h * 2.0);
        let dist_size = Vec2::new(1.0, 1.0);
        draw_texture_ex(self.texture, x, y, WHITE, DrawTextureParams {
            dest_size: Some(dist_size),
            source: Some(src),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        });
    }
}

