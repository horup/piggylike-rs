use std::{sync::Arc, path::{Path, PathBuf}};

use bevy::{asset::{AssetServerSettings, FileAssetIo}, prelude::{Res, AssetServer, Commands, ResMut, Assets}, sprite::TextureAtlas};
use rune::{Module, Sources, Source, Diagnostics, prepare, termcolor::{StandardStream, ColorChoice}, Vm};

use super::api::API;


pub fn setup(mut commands:Commands, asset_server:Res<AssetServer>, mut texture_atlases:ResMut<Assets<TextureAtlas>>) {
    let asset_io = asset_server.asset_io().downcast_ref::<FileAssetIo>().unwrap();
    let mut script_path = asset_io.root_path().clone();
    script_path.push("scripts");
    script_path.push("main.rn");
   

    let mut module = Module::with_crate("engine");
    API::register(&mut module);

    //register_types(&mut module);
    //register_functions(&mut module, commands.clone());

    let mut context = rune_modules::default_context().unwrap();
    context.install(&module).unwrap();
    let mut runtime = Arc::new(context.runtime());

    let mut sources = Sources::new();
    let source = Source::from_path(script_path.as_path()).unwrap();
    sources.insert(source);

    let mut diagnostics = Diagnostics::new();
    let unit = prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources).unwrap();
    }

    let mut vm = Vm::new(runtime, Arc::new(unit.unwrap()));
    
    let mut api = API::default();
    vm.call(&["main"], (&mut api, )).unwrap();
    api.process(&mut commands, &asset_server, &mut texture_atlases);

}