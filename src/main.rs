use assets::AssetsPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use map::MapPlugin;
use tower::TowerPlugin;

mod assets;
mod camera;
mod config;
mod enemy;
mod isometric;
mod map;
mod math;
mod tower;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(AssetsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(TowerPlugin)
        .run();
}
