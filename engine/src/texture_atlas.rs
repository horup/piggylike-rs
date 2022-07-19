use macroquad::prelude::{Texture2D, draw_texture_ex, Color, DrawTextureParams, draw_texture, WHITE, Rect};

pub struct TextureAtlas {
    pub texture:Texture2D,
    pub columns:u16,
    pub rows:u16
}

impl TextureAtlas {
    pub fn new(texture:Texture2D, columns:u16, rows:u16) -> Self {
        Self { texture, columns, rows }
    }

    pub fn draw(&self, index:u32, x:f32, y:f32) {
        let sw = self.texture.width() / self.columns as f32;
        let sh = self.texture.height() / self.rows as f32;
        let sx = index % self.columns as u32;
        let sy = index / self.columns as u32;
        let sx = sx as f32 / self.columns as f32 * self.texture.width();
        let sy = sy as f32 / self.rows as f32 * self.texture.height();

       

        let src = Rect::new(sx, sy, sw, sh);
        draw_texture_ex(self.texture, x, y, WHITE, DrawTextureParams {
            dest_size: None,
            source: Some(src),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        });
    }
}

