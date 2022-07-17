use engine::{macroquad, Engine};
use macroquad::prelude::*;
use engine::macroquad_tiled;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut engine = Engine::default();
    engine.execute_file("assets/scripts/autoexec.rhai").await;
    

    loop {
        clear_background(WHITE);

        if let Some(map) = &engine.world.map {
            let s = if screen_width() < screen_height() { screen_width()} else {screen_height()};
            let dest_rect = Rect::new(0., 0., s,  s);
            map.draw_tiles("tiles", dest_rect, None);
        }
    
        draw_text(&format!("fps:{}", get_fps()), 16.0, 16.0, 16.0, BLACK);
        next_frame().await
    }
}
