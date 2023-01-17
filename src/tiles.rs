use bevy::prelude::*;

use crate::config::{TILE_HEIGHT, TILE_WIDTH};



pub struct TilesPlugin;

#[derive(Component)]
struct Tile {}

#[derive(Resource, Default)]
pub struct SpriteSheets {
    pub map_tiles: Handle<TextureAtlas>,
}

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SpriteSheets::default())
            .add_startup_system_set(
                SystemSet::new()
                    .with_system(make_tiles)
                    // .with_system(generate_grid.after(make_tiles))
            );
    }
}

// fn generate_grid(
//     mut commands: Commands,
//     handles: Res<SpriteSheets>,
//     atlases: Res<Assets<TextureAtlas>>,
// ) {
//     let atlas = atlases.get(&handles.map_tiles).unwrap();
//
//     for x in 0..10 {
//         for y in 0..8 {
//             let pos_x = match y % 2 {
//                 1 => (x as f32) * TILE_WIDTH + (TILE_HEIGHT / 2.0),
//                 _ => (x as f32) * TILE_WIDTH,
//             };
//
//             let pos_y = (y as f32) * (TILE_HEIGHT / 4.0);
//
//             commands.spawn(SpriteBundle {
//                 texture: atlas.texture.clone(),
//                 transform: Transform {
//                     translation: Vec3::new(pos_x as f32, pos_y, 8.0 - y as f32),
//                     ..default()
//                 },
//                 sprite: Sprite {
//                     anchor: Anchor::Center,
//                     rect: Some(Rect::new(16.0, 1.0, 32.0, 17.0)),
//                     ..default()
//                 },
//                 ..default()
//             });
//         }
//     }
// }

pub fn make_tiles(
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
    handles.map_tiles = texture_atlas_handle.clone();
}
