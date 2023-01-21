use bevy::prelude::*;
use assets::AssetsPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use map::MapPlugin;
use tower::TowerPlugin;
use hud::HUDPlugin;

mod assets;
mod camera;
mod config;
mod enemy;
mod isometric;
mod modifier;
mod map;
mod math;
mod tower;
mod debug;
mod hud;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(AssetsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(MapPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(HUDPlugin)
        .run();
}
