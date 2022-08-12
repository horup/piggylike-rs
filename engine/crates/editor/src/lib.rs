use bevy::prelude::*;
use bevy_egui::{egui::{self, TopBottomPanel, menu}, EguiContext};
use metadata::{Metadata, Id};
use tilemap::*;

pub fn setup(mut commands:Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Grid { size: 16 })),
        ..Default::default()
    });
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Tool {
    PlaceTile,
    PlaceThing
}

impl Default for Tool {
    fn default() -> Self {
        Tool::PlaceTile
    }
}

#[derive(Default, Clone)]
pub struct EditorUI {
    pub tool:Tool,
    pub tile:Id
}

#[derive(Default)]
pub struct Editor {
    pub map_width:String,
    pub map_height:String
}

pub fn menu_ui(mut context: ResMut<EguiContext>, mut editor_ui:ResMut<EditorUI>, mut editor:ResMut<Editor>) {
    TopBottomPanel::top("top_0").show(context.ctx_mut(), |ui|{
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
               if ui.button("New").clicked() {

               }
               if ui.button("Save").clicked() {

               }
               if ui.button("Load").clicked() {
                
               }
            });
        });
    });
}

pub fn tools_selection_ui(mut context: ResMut<EguiContext>, mut editor_ui:ResMut<EditorUI>) {
    egui::Window::new("Tools").show(context.ctx_mut(), |ui| {
        ui.radio_value(&mut editor_ui.tool, Tool::PlaceTile, "Place Tile");
        ui.radio_value(&mut editor_ui.tool, Tool::PlaceThing, "Place Thing");
    });
}

pub fn tiles_selection_ui(mut context: ResMut<EguiContext>, metadata:Res<Metadata>, mut editor_ui:ResMut<EditorUI>) {
    egui::Window::new("Tiles").show(context.ctx_mut(), |ui| {
        for (id, tile_def) in metadata.tiles.iter() {
            ui.radio_value(&mut editor_ui.tile, *id, tile_def.name.clone());
        }
    });
}

pub fn properties_ui(mut context: ResMut<EguiContext>, mut tilemap:ResMut<Tilemap>, mut editor_ui:ResMut<EditorUI>, mut editor:ResMut<Editor>) {
    egui::Window::new("Properties").show(context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Width");
            ui.text_edit_singleline(&mut editor.map_width);
        });
        ui.horizontal(|ui| {
            ui.label("Height");
            ui.text_edit_singleline(&mut editor.map_height);
        });
        if ui.button("Save Changes").clicked() {
        }
    });
}

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EditorUI::default());
        app.insert_resource(Editor::default());
        app.add_startup_system(setup);
        app.add_system(tiles_selection_ui.after(tools_selection_ui));
        app.add_system(tools_selection_ui);
        app.add_system(properties_ui.after(tiles_selection_ui));
        app.add_system(menu_ui.before(tools_selection_ui));
    }
}