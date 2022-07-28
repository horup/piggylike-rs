use std::sync::{Arc, Mutex};

use macroquad::prelude::{load_texture, FilterMode};

use crate::{Tile, Thing, Engine, Atlas};

pub enum Command {
    LoadMap {
        path: String,
    },
    DefineTile {
        id: u32,
        tile: Tile,
    },
    DefineAtlas {
        id: u32,
        columns: u16,
        rows: u16,
        texture_path: String,
    },
    DefineThing {
        id: u32,
        thing: Thing,
    },
    LoadWorld {
        file_name: String
    },
    Execute(Box<dyn FnMut(&mut Engine)>)
}

unsafe impl Send for Command {}
unsafe impl Sync for Command {}

#[derive(Clone)]
pub struct Commands {
    commands:Arc<Mutex<Vec<Command>>>
}
unsafe impl Send for Commands {}
unsafe impl Sync for Commands {}

impl Commands {
    pub fn new() -> Self {
        Commands { commands: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn push(&self, command:Command) {
        if let Ok(mut commands) = self.commands.lock() {
            commands.push(command);
        }
    }

    pub fn drain(&self) -> Vec<Command> {
        let mut vec = Vec::new();
        if let Ok(mut commands) = self.commands.lock() {
            vec = commands.drain(..).collect();
        }

        return vec;
    }
}

impl Engine {
    pub async fn process_commands(&mut self) {
        for cmd in self.commands.drain() {
            match cmd {
                Command::LoadMap { path } => {
                    self.load_map(&path).await;
                }
                Command::DefineTile { id, tile } => {
                    self.tile_prototypes.insert(id, tile);
                }
                Command::DefineAtlas {
                    id,
                    columns,
                    rows,
                    texture_path,
                } => {
                    let texture = load_texture(&texture_path).await.unwrap();
                    texture.set_filter(FilterMode::Nearest);
                    self.atlases.insert(
                        id,
                        Atlas {
                            texture,
                            columns,
                            rows,
                        },
                    );
                }
                Command::DefineThing { id, thing } => {
                    self.thing_prototypes.insert(id, thing);
                }
                Command::LoadWorld { file_name } => self.load_world(&file_name),
                Command::Execute(mut f) => {
                    f(self);
                },
            }
        }
    }
}