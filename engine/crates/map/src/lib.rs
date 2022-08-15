use bevy::prelude::*;
use metadata::Id;
use ndarray::{Array2};
use serde::{Serialize, Deserialize};
use tilemap::{Tilemap, Grid};


#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Tile {
    pub floor:Option<Id>,
    pub walls:Option<Id>
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Map {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub tiles: Array2<Tile>,
    pub ambient_light:Color,
    pub ambient_brightness:f32
}

impl Map {
    pub fn save(&self, path:&str) {
        let json = serde_json::to_string(self).unwrap();
        std::fs::write(&path, json).unwrap();
    }

    pub fn load(path:&str) -> Option<Map> {
        if let Ok(json) = std::fs::read_to_string(path) {
            if let Ok(map) = serde_json::from_str::<Map>(&json) {
                return Some(map);
            }
        }
        
        None
    }
}

impl Default for Map {
    fn default() -> Self {
        let size = 16;
        Self {
            name: String::from("Untitled"),
            width: size,
            height: size,
            tiles: Array2::default((size, size)),
            ambient_light:Color::WHITE,
            ambient_brightness:1.0
        }
    }
}


#[derive(Component)]
struct GridEntity;

fn map_changed(mut commands:Commands, map:ResMut<Map>, mut tilemap:ResMut<Tilemap>,  mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, grids:Query<(Entity, &GridEntity)>) {
    if map.is_changed() {
        if map.width * map.height != tilemap.tiles.len() {
            // extend or thrink, todo
        }
        if map.width != tilemap.width as usize || map.height != tilemap.height as usize {
            *tilemap = Tilemap::new(map.width, map.height);

            grids.for_each(|(e,_)| {commands.entity(e).despawn_recursive()});
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(Grid { width: map.width, height: map.height })),
                material: materials.add(StandardMaterial {
                    base_color:Color::WHITE,
                    depth_bias:1000.0,
                    unlit:true,
                    ..Default::default()
                }),
                transform:Transform::from_xyz(0.0, -0.01, 0.0),
                ..Default::default()
            }).insert(GridEntity);
        }

        for ((x,y), tile) in map.tiles.indexed_iter() {
            if let Some(tilemap_tile) = tilemap.tiles.get_mut((x, y)) {
                tilemap_tile.floor = tile.floor;
                tilemap_tile.walls = tile.walls;
            }
        }

        commands.insert_resource(AmbientLight {
            color: map.ambient_light,
            brightness: map.ambient_brightness,
        })
    }
}


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Map::default());
        app.add_system(map_changed);
    }
}