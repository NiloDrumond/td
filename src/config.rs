use bevy::prelude::{MouseButton, SystemLabel};

pub const TILE_WIDTH: f32 = 16.0;
pub const TILE_HEIGHT: f32 = 17.0;
pub const ENEMY_OFFSET: f32 = TILE_WIDTH / 2.0;

pub const MIN_CAMERA_WIDTH: i32 = 160;
pub const MIN_CAMERA_HEIGHT: i32 = 80;
pub const PAN_BUTTON: MouseButton = MouseButton::Middle;
pub const ZOOM_STEP: i32 = 16;

pub const WAYPOINTS_TILESET_NAME: &str =  "mapindicators_high.tsx";



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub enum SysLabel {
    LoadAssets,
    LoadMap,
}
