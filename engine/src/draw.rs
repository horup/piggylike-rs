use macroquad::prelude::*;

use crate::Engine;

impl Engine {
    
    
    pub fn draw_tilemap(&self) {

    }

    pub fn draw_things(&self) {

    }

    pub fn draw_console(&self) {

    }
    
    pub fn draw(&self) {
        clear_background(WHITE);
        self.draw_tilemap();
        self.draw_things();
        self.draw_console();
    }
}