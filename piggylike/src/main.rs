use engine::{macroquad, Engine};
use macroquad::prelude::*;

#[macroquad::main("PiggyLike")]
async fn main() {
    let mut engine = Engine::default();
    engine.eval_file("assets/scripts/autoexec.rhai").await;
    
    let mut console = String::new();
    let mut show_console = true;

    loop {
        clear_background(WHITE);

       /* if let Some(map) = &engine.world.map {
            let s = if screen_width() < screen_height() { screen_width()} else {screen_height()};
            let dest_rect = Rect::new(0., 0., s,  s);
            map.draw_tiles("tiles", dest_rect, None);
        }*/

        let tilemap = &engine.world.tilemap;
        for layer in tilemap.layers.iter() {
            for y in 0..tilemap.height {
                for x in 0..tilemap.width {
                   
                }
            }
        }

    
        if show_console {
            
            if let Some(char) = get_char_pressed() {
                if char.is_ascii_control() == false {
                    console = format!("{}{}", console, char);
                } else {
                    if char == 8 as char {
                        if console.len() > 0 {
                            console = console[0..console.len()-1].into();
                        }
                    } else if char == 13 as char {
                        engine.eval(&console).await;
                        console = "".into();
                    }
                }
            }
            draw_text(&format!("> {}", console), 0.0, 16.0, 16.0, BLACK);
        } else {
            draw_text(&format!("fps:{}", get_fps()), 16.0, 16.0, 16.0, BLACK);
        }
        next_frame().await
    }
}
