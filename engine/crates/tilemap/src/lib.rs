use bevy::prelude::{*, shape::{Cube, Plane}};

mod shape;
use ndarray::Array2;
pub use shape::*;


use bevy::{prelude::Entity};
use serde::{Serialize, Deserialize};

use metadata::{Id, Metadata};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct Tile {
    pub solid:bool,
    pub floor:Option<Id>,
    pub walls:Option<Id>,
    #[serde(skip_serializing)]
    pub entity:Option<Entity>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tilemap {
    pub tiles:Array2<Tile>
}


impl Tilemap {
    pub fn new(width:usize, height:usize) -> Self {
        Self { tiles: Array2::default((width, height)), }
    }

    pub fn width(&self) -> usize {
        self.tiles.dim().0
    }

    pub fn height(&self) -> usize {
        self.tiles.dim().1
    }
}

#[derive(Component)]
struct TileEntity {
    pub x:usize,
    pub y:usize,
    pub floor:Option<Id>,
    pub walls:Option<Id>
}

#[derive(Component)]
struct Floor;

#[derive(Component)]
struct Walls;

fn tilemap_changed(mut commands:Commands,asset_server:Res<AssetServer>, mut tilemap:ResMut<Tilemap>, mut metadata:ResMut<Metadata>, _tile_sprites:Query<(Entity, &TileEntity)>) {
    if tilemap.is_changed() {
        for ((x, y), tile) in tilemap.tiles.indexed_iter_mut() {
            if tile.entity == None {
                tile.entity = Some(commands.spawn().with_children(|f|{
                    f.spawn().insert(Floor);
                    f.spawn().insert(Walls);
                }).id());
            }

            commands.entity(tile.entity.unwrap())
            .insert(TileEntity {
                x,
                y,
                floor: tile.floor,
                walls: tile.walls,
            })
            .insert_bundle(PbrBundle {
                transform: Transform::from_xyz(x as f32, 0.0, y as f32), 
                ..Default::default()
            });
        }
    }
}

fn update_tilemap_entities(meshes:Res<Meshes>, mut materials: ResMut<Assets<StandardMaterial>>,asset_server:Res<AssetServer>, mut commands:Commands, tilemap:Res<Tilemap>, tiles:Query<(Entity, &TileEntity, &Children)>, mut metadata:ResMut<Metadata>) {
    if tilemap.is_changed() == false {
        return;
    }

    let mut get_material = |id:Option<Id>| -> Handle<StandardMaterial> {
        if let Some(id) = id {
            if let Some(def) = metadata.materials.get_mut(&id) {
                if def.handle == None {
                    let material = StandardMaterial {
                        base_color_texture:Some(asset_server.load(&def.base_color_texture)),
                        metallic:0.0,
                        reflectance:0.0,
                        perceptual_roughness:1.0,
                        ..Default::default()
                    };

                    def.handle = Some(materials.add(material));
                }

                return def.handle.clone().unwrap();
            }
        }
    
        Handle::default()
    };

    tiles.for_each(|(entity, tile, children)| {
        let mut delete = false;
        if let Some(tile2) = tilemap.tiles.get((tile.x, tile.y)) {
            if Some(entity) != tile2.entity {
                delete = true;
            } else {
                if let (Some(floor), Some(walls)) = (children.get(0), children.get(1)) {
                    let mut floor = commands.entity(*floor);
                    let y = if tile.walls == None {0.0} else {1.0};
                    floor.insert_bundle(PbrBundle {
                        mesh:meshes.floor.clone(),
                        visibility:Visibility { is_visible: tile2.floor != None },
                        material:get_material(tile.floor),
                        transform:Transform::from_xyz( 0.5, y, 0.5),
                        ..Default::default()
                    });

                    let mut walls = commands.entity(*walls);
                    walls.insert_bundle(PbrBundle {
                        mesh:meshes.walls.clone(),
                        material:get_material(tile.walls),
                        visibility:Visibility { is_visible: tile2.walls != None },
                        transform:Transform::from_xyz( 0.5, 0.5, 0.5),
                        ..Default::default()
                    });
                }
            }
        } else {
            delete = true;
        }

        if delete {
            commands.entity(entity).despawn_recursive();
        }
    });
}

pub struct TilemapPlugin;

pub struct Meshes {
    pub floor:Handle<Mesh>,
    pub walls:Handle<Mesh>
}

pub fn setup(mut commands:Commands, mut meshes:ResMut<Assets<Mesh>>) {
    let floor = Mesh::from(Plane { size: 1.0 });
    let walls = Mesh::from(Cube { size: 1.0});
    let meshes = Meshes {
        floor:meshes.add(floor),
        walls:meshes.add(walls)
    };
    commands.insert_resource(meshes);
}

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Tilemap::default());
        app.add_startup_system(setup);
        app.add_system_to_stage(CoreStage::Update, update_tilemap_entities.after(tilemap_changed));
        app.add_system_to_stage(CoreStage::PreUpdate, tilemap_changed);
    }
}