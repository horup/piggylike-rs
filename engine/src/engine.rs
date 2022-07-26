
pub use macroquad;
use macroquad::prelude::{load_string, load_texture, FilterMode, Vec2, get_frame_time};
pub use macroquad_tiled;
use std::borrow::BorrowMut;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{cell::RefCell, collections::HashMap};

use crate::{Input, Thing, Sprite, Atlas, World, Command, Tile};

pub type Commands = Arc<Mutex<Vec<Command>>>;

pub struct Engine {
    pub vm:rune::Vm,
    pub callbacks:HashMap<String, Vec<String>>,
    pub timeline:Vec<World>,
    pub input:Input,
    pub thing_prototypes: HashMap<u32, Thing>,
    pub sprite_prototypes: HashMap<u32, Sprite>,
    pub atlases: HashMap<u32, Atlas>,
    pub tile_prototypes: HashMap<u32, Tile>,
    pub world: World,
    pub commands: Commands,
}


impl Engine {
    pub fn new(script_path:&Path) -> Self {
        let commands = Arc::new(Mutex::new(Vec::new()));
        let vm = Self::vm_create(script_path, commands.clone());
        Self {
            vm:vm,
            callbacks: HashMap::new(),
            timeline:Default::default(),
            input:Input::default(),
            thing_prototypes: HashMap::new(),
            sprite_prototypes: HashMap::new(),
            atlases: HashMap::new(),
            tile_prototypes: HashMap::new(),
            world: Default::default(),
            commands:commands.clone(),
        }
    }

    pub fn warn(&self, warn:&str) {
        println!("warning: {}", warn);
    }

    pub async fn update(&mut self) {
        self.update_cleanup();
        self.update_input();
        self.update_movement();
        self.process_commands().await;
        self.draw();
        self.world.iterations += 1;
        self.update_history();
    }

    pub fn get_delta_time(&self) -> f32 {
        let dt = get_frame_time();
        let max = 0.1;
        if dt < max {
            return dt;
        } else {
            return max;
        }
    }
}

