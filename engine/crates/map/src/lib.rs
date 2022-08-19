use bevy::prelude::*;
use metadata::Id;
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use tilemap::{Grid, Tilemap};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Tile {
    pub top: f32,
    pub bottom: f32,
    pub floor: Id,
    pub walls: Id,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Map {
    pub name: String,
    pub tiles: Array2<Tile>,
    pub ambient_light: Color,
    pub ambient_brightness: f32,
}

impl Map {
    pub fn test_8x8() -> Self {
        let size = 8;
        let mut tiles: Array2<Tile> = Array2::default((size, size));
        for y in 1..size - 1 {
            for x in 1..size - 1 {
                tiles[(x, y)].top = 1.0;
            }
        }

        tiles[(size / 2, size / 2)].top = 0.0;
        Self {
            name: "Test Map".into(),
            tiles,
            ambient_light: Color::WHITE,
            ambient_brightness: 1.0,
        }
    }

    pub fn save(&self, path: &str) {
        let json = serde_json::to_string(self).unwrap();
        std::fs::write(&path, json).unwrap();
    }

    pub fn load(path: &str) -> Option<Map> {
        if let Ok(json) = std::fs::read_to_string(path) {
            if let Ok(map) = serde_json::from_str::<Map>(&json) {
                return Some(map);
            }
        }

        None
    }

    pub fn width(&self) -> usize {
        self.tiles.dim().0
    }

    pub fn height(&self) -> usize {
        self.tiles.dim().1
    }

    pub fn resize(&mut self, w: usize, h: usize) {
        let mut tiles: Array2<Tile> = Array2::default((w, h));
        for y in 0..tiles.dim().1 {
            for x in 0..tiles.dim().0 {
                if let Some(tile) = self.tiles.get((x, y)) {
                    tiles[(x, y)] = tile.clone();
                }
            }
        }

        self.tiles = tiles;
    }
}

impl Default for Map {
    fn default() -> Self {
        let size = 16;
        Self {
            name: String::from("Untitled"),
            tiles: Array2::default((size, size)),
            ambient_light: Color::WHITE,
            ambient_brightness: 1.0,
        }
    }
}

#[derive(Component)]
struct GridEntity;

fn map_changed(
    mut commands: Commands,
    map: ResMut<Map>,
    mut tilemaps: Query<(&mut Tilemap)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grids: Query<(Entity, &GridEntity)>,
) {
    if map.is_changed() {
        for (mut tilemap) in tilemaps.iter_mut() {
            if map.width() != tilemap.width() || map.height() != tilemap.height() {
                *tilemap = Tilemap::new(map.width(), map.height());

                grids.for_each(|(e, _)| commands.entity(e).despawn_recursive());
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(Grid {
                            width: map.width(),
                            height: map.height(),
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::WHITE,
                            depth_bias: 1000.0,
                            unlit: true,
                            ..Default::default()
                        }),
                        transform: Transform::from_xyz(0.0, -0.01, 0.0),
                        ..Default::default()
                    })
                    .insert(GridEntity);
            }

            for ((x, y), tile) in map.tiles.indexed_iter() {
                if let Some(tilemap_tile) = tilemap.tiles.get_mut((x, y)) {
                    tilemap_tile.bottom = tile.bottom;
                    tilemap_tile.top = tile.top;
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
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Map::test_8x8());
        app.add_system(map_changed);
    }
}
