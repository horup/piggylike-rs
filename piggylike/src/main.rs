use kernel::macroquad;
use macroquad::prelude::*;
use kernel::Tilemap;


#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        let tilemap = Tilemap {

        };
        
        clear_background(WHITE);

        draw_line(42.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS! 123", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

