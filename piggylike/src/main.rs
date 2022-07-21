#![allow(warnings, unused)]

use engine::{macroquad, Engine, Atlas};
use macroquad::prelude::*;

struct PlayerInput {
    pub x:f32,
    pub y:f32,
}

#[macroquad::main("PiggyLike")]
async fn main() {
    let mut engine = Engine::default();
    engine.eval_file("assets/scripts/autoexec.rhai").await;

    let mut console = String::new();
    let show_console = false;


    loop {
        let dt = get_frame_time();
        let input = PlayerInput {
            x:if is_key_down(KeyCode::A) {-1.0} else if is_key_down(KeyCode::D) {1.0} else {0.0},
            y:if is_key_down(KeyCode::W) {-1.0} else if is_key_down(KeyCode::S) {1.0} else {0.0},
        };

        clear_background(WHITE);
        let aspect = screen_width() / screen_height();
        let mut offset = Vec2::new(0.0, 0.0);
        let zoom = 1.0/16.0;



        if let Some((_, player)) = engine.world.things.iter_mut().find(|(_, thing)| {
            thing.player
        }) {
            let speed = 10.0;
            player.pos.x += input.x * dt * speed;
            player.pos.y += input.y * dt * speed;
            offset.x = player.pos.x;
            offset.y = player.pos.y;
        }

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
                        if let Some(atlas) = engine.atlases.get(&tile.atlas) {
                            atlas.draw(tile.atlas_index, x, y);
                        }
                    }
                }
            }
        }

        for (_, thing) in engine.world.things.iter() {
            if let Some(atlas) = engine.atlases.get(&thing.atlas) {
                atlas.draw(thing.atlas_index, thing.pos.x, thing.pos.y);
            }
        }

        set_default_camera();
    
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
