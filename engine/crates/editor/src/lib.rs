use bevy::prelude::*;
use bevy_egui::{
    egui::{self, menu, TopBottomPanel, Ui},
    EguiContext,
};
use map::{Map, MapPlugin};
use metadata::{Id, Metadata};
use smart_camera::WorldCursor;

pub fn setup(
    _commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
) {
    /*  commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Grid { size: 16 })),
        material: materials.add(StandardMaterial {
            base_color:Color::WHITE,
            depth_bias:1000.0,
            unlit:true,
            ..Default::default()
        }),
        transform:Transform::from_xyz(0.0, -0.01, 0.0),
        ..Default::default()
    });*/
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Tool {
    TileEditing,
    ThingEditing,
}

impl ToString for Tool {
    fn to_string(&self) -> String {
        match self {
            Tool::TileEditing => "Tile".into(),
            Tool::ThingEditing => "Thing".into(),
        }
    }
}

impl Default for Tool {
    fn default() -> Self {
        Tool::TileEditing
    }
}

#[derive(Default, Clone, Copy)]
pub struct Tile {
    pub floor: Id,
    pub ceiling: Id,
    pub walls: Id,
    pub height: f32,
}

#[derive(Clone)]
pub struct Editor {
    pub tool: Tool,
    pub ambient_brightness: f32,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
    pub tile_index: usize,
}

impl Default for Editor {
    fn default() -> Self {
        let mut tiles = Vec::new();
        for _ in 0..8 {
            tiles.push(Tile::default())
        }
        Self {
            tool: Default::default(),
            ambient_brightness: Default::default(),
            width: Default::default(),
            height: Default::default(),
            tiles: tiles,
            tile_index: Default::default(),
        }
    }
}

pub fn toolbox_ui(
    mut context: ResMut<EguiContext>,
    mut editor: ResMut<Editor>,
    _map: ResMut<Map>,
    metadata: Res<Metadata>,
) {
    egui::SidePanel::left("my_left_panel").show(context.ctx_mut(), |ui| {
        ui.label("Tool selection");
        egui::ComboBox::from_label("Tool")
            .selected_text(editor.tool.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut editor.tool,
                    Tool::ThingEditing,
                    Tool::ThingEditing.to_string(),
                );
                ui.selectable_value(
                    &mut editor.tool,
                    Tool::TileEditing,
                    Tool::TileEditing.to_string(),
                );
            });

        ui.separator();
        ui.label("Tile editing");
        egui::ComboBox::from_label("Tile")
            .selected_text(editor.tile_index.to_string())
            .show_ui(ui, |ui| {
                for index in 0..editor.tiles.len() {
                    ui.selectable_value(&mut editor.tile_index, index, index.to_string());
                }
            });
        let index = editor.tile_index;
        if let Some(tile) = editor.tiles.get_mut(index) {
            egui::ComboBox::from_label("Ceiling")
                .selected_text(metadata.material(&tile.ceiling).name)
                .show_ui(ui, |ui| {
                    for (id, material) in metadata.materials.iter() {
                        ui.selectable_value(&mut tile.ceiling, *id, material.name.clone());
                    }
                });
            egui::ComboBox::from_label("Walls")
                .selected_text(metadata.material(&tile.walls).name)
                .show_ui(ui, |ui| {
                    for (id, material) in metadata.materials.iter() {
                        ui.selectable_value(&mut tile.walls, *id, material.name.clone());
                    }
                });
            egui::ComboBox::from_label("Floor")
                .selected_text(metadata.material(&tile.floor).name)
                .show_ui(ui, |ui| {
                    for (id, material) in metadata.materials.iter() {
                        ui.selectable_value(&mut tile.floor, *id, material.name.clone());
                    }
                });

            egui::ComboBox::from_label("Height")
                .selected_text(format!("{:?}", tile.height))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut tile.height, 0.0, "0.0");
                    ui.selectable_value(&mut tile.height, 1.0, "1.0");
                    ui.selectable_value(&mut tile.height, 2.0, "2.0");
                });
        }
    });
}

pub fn menu_ui(mut context: ResMut<EguiContext>, _editor_ui: ResMut<Editor>, mut map: ResMut<Map>) {
    TopBottomPanel::top("top_0").show(context.ctx_mut(), |ui| {
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {}
                if ui.button("Save").clicked() {
                    map.save("somemap.json");
                }
                if ui.button("Load").clicked() {
                    *map = Map::load("somemap.json").unwrap();
                }
            });
        });
    });
}

pub fn tools_selection_ui(mut context: ResMut<EguiContext>, mut editor: ResMut<Editor>) {
    egui::Window::new("Tools").show(context.ctx_mut(), |ui| {
        ui.radio_value(&mut editor.tool, Tool::TileEditing, "Tile");
        ui.radio_value(&mut editor.tool, Tool::ThingEditing, "Thing");
    });
}

fn usize_edit_single(ui: &mut Ui, value: &mut usize) {
    let mut s = value.to_string();
    ui.text_edit_singleline(&mut s);
    if let Ok(v) = usize::from_str_radix(&s, 10) {
        *value = v;
    }
}

fn f32_edit_single(ui: &mut Ui, value: &mut f32) {
    let mut s = value.to_string();
    ui.text_edit_singleline(&mut s);
    if let Ok(v) = s.parse::<f32>() {
        *value = v;
    }
}

pub fn map_ui(mut context: ResMut<EguiContext>, mut editor: ResMut<Editor>, mut map: ResMut<Map>) {
    egui::Window::new("Map").show(context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Name");
        });
        ui.horizontal(|ui| {
            ui.label("Width");
            usize_edit_single(ui, &mut editor.width);
        });
        ui.horizontal(|ui| {
            ui.label("Height");
            usize_edit_single(ui, &mut editor.height);
        });
        ui.horizontal(|ui| {
            ui.label("Ambient");
            f32_edit_single(ui, &mut editor.ambient_brightness);
        });
        if ui.button("Save Changes").clicked() {
            map.ambient_brightness = editor.ambient_brightness;
            map.resize(editor.width, editor.height);
        }
    });
}

pub fn cursor(
    world_cursor: Res<WorldCursor>,
    mut map: ResMut<Map>,
    mouse_buttons: Res<Input<MouseButton>>,
    editor: Res<Editor>,
    time: Res<Time>
) {
    let place = mouse_buttons.pressed(MouseButton::Left);
    let remove = mouse_buttons.pressed(MouseButton::Right);
    if place || remove {
        let x = world_cursor.position.x as i32;
        let y = world_cursor.position.z as i32;
        if x >= 0 && y >= 0 {
            let mut map_clone = map.clone();
            if let Some(cell) = map_clone.tiles.get_mut((x as usize, y as usize)) {
                if place {
                    if let Some(tile) = editor.tiles.get(editor.tile_index) {
                        *cell = map::Tile {
                            top: tile.height,
                            bottom: 0.0,
                            floor_material: tile.floor,
                            wall_material: tile.walls,
                            ceiling_material: tile.ceiling,
                        }
                    }
                } else if remove {
                    *cell = map::Tile::default();
                }
            }

            if map.ne(&map_clone) {
                *map = map_clone;
            }
        }
    }
}

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MapPlugin);
        app.insert_resource(Editor::default());
        app.add_startup_system(setup);

        app.add_system(cursor);
        app.add_system(menu_ui.after(cursor));

        app.add_system(toolbox_ui.after(menu_ui));
        app.add_system(map_ui.after(toolbox_ui));
    }
}
