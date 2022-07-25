
pub use macroquad;
use macroquad::prelude::{load_string, load_texture, FilterMode, Vec2, get_frame_time};
pub use macroquad_tiled;
use std::borrow::BorrowMut;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use crate::{Input, Thing, Sprite, Atlas, World, Command, Tile};

pub struct Engine {
    pub timeline:Vec<World>,
    pub input:Input,
    pub thing_prototypes: HashMap<u32, Thing>,
    pub sprite_prototypes: HashMap<u32, Sprite>,
    pub atlases: HashMap<u32, Atlas>,
    pub tile_prototypes: HashMap<u32, Tile>,
    pub world: World,
    pub script_engine: rhai::Engine,
    pub script: rhai::AST,
    pub global_scope: rhai::Scope<'static>,
    pub commands: Rc<RefCell<Vec<Command>>>,
}


impl Default for Engine {
    fn default() -> Self {
        let commands = Rc::new(RefCell::new(Vec::new()));
        Self {
            global_scope: rhai::Scope::new(),
            timeline:Default::default(),
            input:Input::default(),
            thing_prototypes: HashMap::new(),
            sprite_prototypes: HashMap::new(),
            atlases: HashMap::new(),
            tile_prototypes: HashMap::new(),
            world: Default::default(),
            script_engine: Self::new_script_engine(commands.clone()),
            commands:commands.clone(),
            script:rhai::AST::default()
        }
    }
}


impl Engine {
    pub fn warn(&self, warn:&str) {
        println!("warning: {}", warn);
    }

    pub async fn update(&mut self) {
        self.update_input();
        self.update_movement();
        self.update_script().await;
        self.process_commands().await;

        


        self.draw();
        self.world.iterations += 1;
        self.update_cleanup();
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

