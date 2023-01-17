use bevy::prelude::*;
use camera::CameraPlugin;
use map::MapPlugin;
use tiles::TilesPlugin;

mod camera;
mod tiles;
mod map;
mod config;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(CameraPlugin)
        .add_plugin(TilesPlugin)
        .add_plugin(MapPlugin)
        .run();
}

