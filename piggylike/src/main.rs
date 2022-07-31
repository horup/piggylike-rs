#![allow(warnings, unused)]

use engine::{bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}}, EnginePlugin, resources::Config};


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
   // .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(EnginePlugin)
    .insert_resource(Config {
        pixel_snap_resolution: 1.0/48.0,
    })
    .run();
}
/*
use std::path::Path;

use engine::{macroquad, Engine, Atlas};
use macroquad::prelude::*;

struct PlayerInput {
    pub x:f32,
    pub y:f32,
}

#[macroquad::main("PiggyLike")]
async fn main() {
    let mut engine = Engine::new(&Path::new("assets/scripts/main.rn"));

    let mut console = String::new();
    let show_console = false;


    loop {
        engine.update().await;

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
                        //engine.eval(&console).await;
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
}*/
