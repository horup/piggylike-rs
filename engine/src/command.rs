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
    Execute {
        function:Box<dyn FnMut(&mut Engine)>
    }
}

unsafe impl Send for Command {}
unsafe impl Sync for Command {}

impl Engine {

    pub fn push_command(&self, command:Command) {
        //self.commands.borrow_mut().push(command);
    }
    pub async fn process_commands(&mut self) {
       /* for cmd in self.commands.clone().as_ref().borrow_mut().drain(..) {
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
                Command::Execute { mut function } => {
                    function(self);
                },
            }
        }*/
    }
}