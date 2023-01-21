use bevy::prelude::*;

use crate::config::{TILE_HEIGHT, TILE_WIDTH, STATBAR_HEIGHT, STATBAR_WIDTH};

#[derive(Resource, Default)]
pub struct SpriteSheets {
    pub map_sprites: Handle<TextureAtlas>,
    pub enemy_sprites: Handle<TextureAtlas>,
    pub indicators_sprites: Handle<TextureAtlas>,
    pub statbar_sprites: Handle<TextureAtlas>,
}

pub fn load_enemy_sheet(
    mut handles: ResMut<SpriteSheets>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tiles_path = "IsometricTRPGAssetPack_Entities.png".to_string();

    let texture_handle = asset_server.load(&tiles_path);

    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_WIDTH, TILE_HEIGHT),
        4,
        33,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    handles.enemy_sprites = texture_atlas_handle.clone();
}

pub fn load_map_sheet(
    mut handles: ResMut<SpriteSheets>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tiles_path = "Isometric_MedievalFantasy_Tiles.png".to_string();
    let texture_handle = asset_server.load(&tiles_path);

    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_WIDTH, TILE_HEIGHT),
        11,
        10,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    handles.map_sprites = texture_atlas_handle.clone();
}
pub fn load_indicators_sheet(
    mut handles: ResMut<SpriteSheets>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tiles_path = "mapindicators_low.png".to_string();
    let texture_handle = asset_server.load(&tiles_path);

    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_WIDTH, TILE_HEIGHT),
        2,
        3,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    handles.indicators_sprites = texture_atlas_handle.clone();
}

pub fn load_statbar_sheet(
    mut handles: ResMut<SpriteSheets>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tiles_path = "statbar.png".to_string();
    let texture_handle = asset_server.load(&tiles_path);

    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(STATBAR_WIDTH, STATBAR_HEIGHT),
        1,
        2,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    handles.statbar_sprites = texture_atlas_handle.clone();
}
