use bevy::prelude::*;
use bevy_egui::{
    egui::{self, menu, TopBottomPanel, Ui},
    EguiContext,
};
use map::{Map, MapPlugin};
use metadata::{Id, Metadata};
use smart_camera::WorldCursor;
use tilemap::*;

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
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
    PlaceTile,
    PlaceThing,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::PlaceTile
    }
}

#[derive(Default, Clone)]
pub struct Editor {
    pub tool: Tool,
    pub floor: Id,
    pub ambient_brightness:f32,
    pub width:usize,
    pub height:usize
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
        ui.radio_value(&mut editor.tool, Tool::PlaceTile, "Place Tile");
        ui.radio_value(&mut editor.tool, Tool::PlaceThing, "Place Thing");
    });
}

pub fn material_selection_ui(
    mut context: ResMut<EguiContext>,
    metadata: Res<Metadata>,
    mut editor_ui: ResMut<Editor>,
) {
    egui::Window::new("Materials").show(context.ctx_mut(), |ui| {
        for (id, material_def) in metadata.materials.iter() {
            ui.radio_value(&mut editor_ui.floor, *id, material_def.name.clone());
        }
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

pub fn map_ui(
    mut context: ResMut<EguiContext>,
    _tilemap: ResMut<Tilemap>,
    mut editor: ResMut<Editor>,
    mut map: ResMut<Map>,
) {
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
            map.width = editor.width;
            map.height = editor.height;
        }
    });

}

pub fn cursor(
    world_cursor: Res<WorldCursor>,
    mut map: ResMut<Map>,
    mouse_buttons: Res<Input<MouseButton>>,
    editor: Res<Editor>,
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
                    *cell = map::Tile {
                        floor: Some(editor.floor),
                        walls: None
                    };
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
        app.add_system(material_selection_ui.after(tools_selection_ui));
        app.add_system(tools_selection_ui);
        app.add_system(map_ui.after(material_selection_ui));
        app.add_system(menu_ui.before(tools_selection_ui));
        app.add_system(cursor.after(tools_selection_ui));
    }
}
