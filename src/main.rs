use assets::AssetsPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use map::MapPlugin;
use enemy::EnemyPlugin;

mod camera;
mod enemy;
mod map;
mod config;
mod math;
mod isometric;
mod assets;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(AssetsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(EnemyPlugin)
        .run();
}

