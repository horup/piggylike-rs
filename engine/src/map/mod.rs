use bevy::{prelude::*, asset::FileAssetIo};
use tiled::*;
use std::path::PathBuf;

pub fn get_assets_path(world:&World) -> PathBuf {
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let asset_io = asset_server.asset_io().downcast_ref::<FileAssetIo>().unwrap();
    let assets_path = asset_io.root_path().clone();
    return assets_path;
}

pub fn load_map(world:&mut World, map_path:&str) -> Result<()> {
    let mut loader = Loader::new();
    let mut path = get_assets_path(world).clone();
    path.push(PathBuf::from(map_path));
    let map = loader.load_tmx_map(path)?;

    println!("{:?}", map);
    Ok(())
}