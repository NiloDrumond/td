use bevy::{prelude::*, sprite::Anchor};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader};

use crate::{
    config::{TILE_HEIGHT, TILE_WIDTH},
    tiles::{make_tiles, SpriteSheets},
};

pub struct MapPlugin;

#[derive(Serialize, Deserialize, Debug)]
struct MapLayer {
    data: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MapData {
    height: usize,
    width: usize,
    layers: Vec<MapLayer>,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(render_map.after(make_tiles));
    }
}

fn parse_map() -> Result<MapData, Box<dyn Error>> {
    let file = File::open("assets/test.json")?;
    let reader = BufReader::new(file);
    let parsed = serde_json::from_reader(reader)?;
    Ok(parsed)
}

fn render_map(mut commands: Commands, handles: Res<SpriteSheets>) {
    let map = parse_map().unwrap();

    let handle = &handles.map_tiles;

    let layer = &map.layers[0].data;
    let height_offset = ((map.width + map.height) as f32 / 4.0) * TILE_HEIGHT;
    for (index, sprite_index) in layer.iter().enumerate() {
        if *sprite_index != 0 {
            let x: isize = (index % map.width).try_into().unwrap();
            let y: isize = (index / map.width).try_into().unwrap();
            let pos_x = (x - y) as f32 * (TILE_WIDTH / 2.0);
            let pos_y = -(x + y) as f32 * (TILE_WIDTH / 4.0);

            commands.spawn(SpriteSheetBundle {
                texture_atlas: handle.clone(),
                sprite: TextureAtlasSprite {
                    anchor: Anchor::BottomCenter,
                    index: *sprite_index as usize - 1,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(pos_x, pos_y + height_offset, 1.0),
                    ..default()
                },
                ..default()
            });
        }
    }
}
