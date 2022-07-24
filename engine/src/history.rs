use std::{fs::{read_to_string, create_dir_all}, path::Path};
use crate::{Engine, World};

impl Engine {
    pub fn save_world(&self, file_name:&str) {
        if let Ok(s) = serde_json::to_string(&self.world) {
            if Path::new("saves").exists() == false {
                create_dir_all("saves");
            }
            
            std::fs::write(&format!("saves/{}", file_name), s);
        }
    }

    pub fn load_world(&mut self, file_name:&str) {
        if let Ok(s) = read_to_string(&format!("saves/{}", file_name)) {
            match serde_json::from_str::<World>(&s) {
                Ok(world) => self.world = world,
                Err(err) => println!("{:?}", err),
            }
        }
    }

    pub fn push_timeline(&mut self) {
        self.timeline.push(self.world.clone());
    }

    pub fn pop_timeline(&mut self) {
        if let Some(world) = self.timeline.pop() {
            self.world = world;
        }
    }

    pub fn update_history(&mut self) {
        if self.world.iterations % 10 == 0 {
            self.push_timeline();
        }
    }

}