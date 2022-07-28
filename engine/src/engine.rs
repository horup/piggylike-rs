use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, asset::FileAssetIo};

use crate::ScriptPlugin;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases:ResMut<Assets<TextureAtlas>>) {
    
    let texture_handle = asset_server.load("textures/tiles.png");
    let mut texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16);
    texture_atlas.size = Vec2::new(1.0, 1.0);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1.0/16.0;
    commands.spawn_bundle(camera_bundle);
    let size = 256;
    for y in 0..size {
        for x in 0..size {
            commands.spawn_bundle(SpriteSheetBundle {
                sprite:TextureAtlasSprite {
                    index:x % 17 as usize,
                    custom_size:Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle.clone(),
                transform:Transform {
                    translation:Vec3::new(x as f32, y as f32, 0.0),
                    ..Default::default()
                },
                ..default()
            });
        }
    } 
   
}

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        let p = env!("CARGO_MANIFEST_DIR");
        println!("{}", p);
        app.add_startup_system(setup)
        .add_plugin(ScriptPlugin);
    }
}

