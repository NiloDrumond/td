use bevy::{prelude::*, sprite::Anchor, utils::HashMap};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader};

use crate::{
    assets::SpriteSheets,
    camera::MouseWorldPos,
    config::{SysLabel, TILE_WIDTH, WAYPOINTS_TILESET_NAME, TILE_HEIGHT_OFFSET},
    isometric::{
        coordinates_to_screen, screen_to_coordinates, Coordinates, ScreenCoordinates,
        SpriteDirection,
    },
};

pub struct MapPlugin;

struct ProtoWaypoint {
    coordinates: Coordinates,
    screen_coordinates: ScreenCoordinates,
}

#[derive(Debug)]
pub struct Waypoint {
    pub coordinates: Coordinates,
    pub screen_coordinates: ScreenCoordinates,
    pub direction: SpriteDirection,
}

pub struct MapPath {
    pub waypoints: Vec<Waypoint>,
    pub id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct MapLayer {
    data: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MapTileset {
    firstgid: usize,
    source: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapData {
    height: usize,
    width: usize,
    tilesets: Vec<MapTileset>,
    layers: Vec<MapLayer>,
}

#[derive(Resource)]
pub struct CurrentMap {
    pub data: MapData,
    pub path: MapPath,
    pub height_offset: f32,
}

#[derive(Component)]
struct CursorIndicator;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            load_map
                .after(SysLabel::LoadAssets)
                .label(SysLabel::LoadMap),
        )
        .add_startup_system(render_map.after(SysLabel::LoadMap))
        .add_startup_system(render_cursor)
        .add_system(update_cursor);
    }
}

fn parse_map() -> Result<MapData, Box<dyn Error>> {
    let file = File::open("assets/waypoints.json")?;
    let reader = BufReader::new(file);
    let parsed = serde_json::from_reader(reader)?;
    Ok(parsed)
}

fn sort_waypoints(
    waypoints_map: &mut HashMap<usize, Coordinates>,
    height_offset: f32,
) -> Vec<Waypoint> {
    let mut proto_waypoints: Vec<ProtoWaypoint> = vec![];
    for i in 0..99 {
        let index = i as usize;
        let Some(coordinates) = waypoints_map.remove(&index) else {
            break;
        };
        let ScreenCoordinates { x, y } = coordinates_to_screen(&coordinates);
        proto_waypoints.push(ProtoWaypoint {
            coordinates,
            screen_coordinates: ScreenCoordinates {
                x,
                y: y + height_offset,
            },
        })
    }
    let mut waypoints: Vec<Waypoint> = proto_waypoints
        .windows(2)
        .map(|window| {
            let [first, second] = window else {
            panic!();
        };
            Waypoint {
                coordinates: first.coordinates,
                screen_coordinates: first.screen_coordinates,
                direction: SpriteDirection::get_direction(first.coordinates, second.coordinates),
            }
        })
        .collect();
    let last = proto_waypoints.last().unwrap();
    let penul = waypoints.last().unwrap();
    waypoints.push(Waypoint {
        coordinates: last.coordinates,
        screen_coordinates: last.screen_coordinates,
        direction: penul.direction,
    });
    waypoints
}

fn load_map(mut commands: Commands) {
    let map = parse_map().unwrap();

    let Some(waypoint_layer_idx) = map.tilesets.iter().position(|tileset| tileset.source == WAYPOINTS_TILESET_NAME) else {
        panic!();
    };
    let waypoint_layer = &map.layers[waypoint_layer_idx];
    let first_waypoint = &map.tilesets[waypoint_layer_idx].firstgid;

    let height_offset = ((map.width + map.height) as f32 / 8.0) * TILE_WIDTH;
    let mut waypoints_map: HashMap<usize, Coordinates> = HashMap::new();
    for (index, sprite_index) in waypoint_layer.data.iter().enumerate() {
        if *sprite_index != 0 {
            let x: usize = (index % map.width).try_into().unwrap();
            let y: usize = (index / map.width).try_into().unwrap();
            let waypoint_index = sprite_index - first_waypoint;
            waypoints_map.insert(waypoint_index, Coordinates(x, y));
        }
    }

    let path = sort_waypoints(&mut waypoints_map, height_offset);
    let current_map = CurrentMap {
        height_offset,
        path: MapPath {
            waypoints: path,
            id: 0,
        },
        data: map,
    };
    commands.insert_resource(current_map);
}

fn render_map(mut commands: Commands, handles: Res<SpriteSheets>, current_map: Res<CurrentMap>) {
    let handle = &handles.map_sprites;

    let map = &current_map.data;
    let layer = &current_map.data.layers[0].data;
    let height_offset = current_map.height_offset;

    for (index, sprite_index) in layer.iter().enumerate() {
        if *sprite_index != 0 {
            let x: usize = (index % map.width).try_into().unwrap();
            let y: usize = (index / map.width).try_into().unwrap();
            let Vec2 {
                x: screen_x,
                y: screen_y,
            } = coordinates_to_screen(&Coordinates(x, y));

            commands.spawn(SpriteSheetBundle {
                texture_atlas: handle.clone(),
                sprite: TextureAtlasSprite {
                    anchor: Anchor::Center,
                    index: *sprite_index as usize - 1,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(screen_x, screen_y + height_offset, 1.0),
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn render_cursor(mut commands: Commands, handles: Res<SpriteSheets>) {
    let handle = &handles.indicators_sprites;

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: handle.clone(),
            sprite: TextureAtlasSprite {
                anchor: Anchor::Center,
                index: 4,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(9999.0, 9999.0, 2.0),
                ..default()
            },
            ..default()
        })
        .insert(CursorIndicator);
}

fn update_cursor(
    mouse_pos: Res<MouseWorldPos>,
    mut query: Query<&mut Transform, With<CursorIndicator>>,
    current_map: Res<CurrentMap>,
) {
    let height_offset = current_map.height_offset;
    let total_offset = height_offset + TILE_HEIGHT_OFFSET;
    let mut transform = query.single_mut();
    let iso_position = screen_to_coordinates(&ScreenCoordinates::new(
        mouse_pos.x,
        mouse_pos.y - total_offset ,
    ));
    let ScreenCoordinates { x, y } = coordinates_to_screen(&iso_position);
    transform.translation = Vec3::new(x, y + total_offset, 2.0);
}
