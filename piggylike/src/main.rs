#![allow(warnings, unused)]

use engine::{macroquad, Engine, Atlas};
use macroquad::prelude::*;

#[macroquad::main("PiggyLike")]
async fn main() {
    let mut engine = Engine::default();
    engine.eval_file("assets/scripts/autoexec.rhai").await;

    let mut console = String::new();
    let show_console = true;


    loop {
        clear_background(WHITE);
        let aspect = screen_width() / screen_height();
        let offset = Vec2::new(16.0, 16.0);
        let zoom = 1.0/16.0;
        let camera = Camera2D {
            zoom:Vec2::new(zoom, -zoom * aspect),
            target:offset,
            ..Default::default()
        };

        set_camera(&camera);

        let tilemap = &engine.world.tilemap;
        for layer in tilemap.layers.iter() {
            for y in 0..tilemap.height {
                for x in 0..tilemap.width {
                   
                    if let Some(tile) = tilemap.get(0, x, y) {
                        let x = x as f32;
                        let y = y as f32;
                        if let Some(atlas) = engine.texture_atlases.get(&tile.atlas) {
                            atlas.draw(tile.atlas_index, x, y);
                        }
                    }
                }
            }
        }

        for i in 0..4 {
            //atlas.draw(i + 32, i as f32 * 16.0, 0.0);
        }


        set_default_camera();



        //atlas.draw(0, 16.0, 32.0);
    
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
