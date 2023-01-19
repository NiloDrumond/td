use bevy::{prelude::*, sprite::Anchor};

use crate::{
    assets::SpriteSheets,
    config::{SysLabel, TILE_WIDTH},
    isometric::{coordinates_to_screen, Coordinates},
    map::CurrentMap,
    math::{move_towards},
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(test_spawn.after(SysLabel::LoadMap))
            .add_system(move_enemies);
    }
}

#[derive(Component)]
pub struct Enemy {
    speed: f32,
    sprite_index: usize,
    position: Coordinates,
    last_waypoint: usize,
}

fn test_spawn(commands: Commands, handles: Res<SpriteSheets>, map: Res<CurrentMap>) {
    let enemy_sprites = &handles.enemy_sprites;
    let waypoint = &map.path.waypoints[0];
    spawn_enemy(
        commands,
        waypoint.coordinates.clone(),
        32,
        enemy_sprites,
        map,
    );
}

fn move_enemies(mut enemies_query: Query<(&mut Transform, &mut Enemy)>, map: Res<CurrentMap>) {
    let waypoints = &map.path.waypoints;
    for (mut transform, mut enemy) in enemies_query.iter_mut() {
        // let direction = waypoints[enemy.last_waypoint].direction;
        let next_coords = waypoints[enemy.last_waypoint + 1].coordinates;
        let Vec2 { x, y } = coordinates_to_screen(&next_coords);
        let next_waypoint = Vec3::new(x, y, transform.translation.z);
        transform.translation = move_towards(
            transform.translation,
            next_waypoint,
            enemy.speed,
        );
        if transform.translation == next_waypoint {
            enemy.last_waypoint += 1;
        }
    }
}

fn spawn_enemy(
    mut commands: Commands,
    position: Coordinates,
    sprite_index: usize,
    texture_atlas: &Handle<TextureAtlas>,
    current_map: Res<CurrentMap>,
) -> Entity {
    let Vec2 {
        x: screen_x,
        y: screen_y,
    } = coordinates_to_screen(&position);
    let screen_y = screen_y + TILE_WIDTH / 2.0;
    let height_offset = current_map.height_offset;

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                anchor: Anchor::BottomCenter,
                index: sprite_index,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(screen_x, screen_y + height_offset, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Enemy {
            sprite_index,
            position,
            speed: 0.1,
            last_waypoint: 0,
        })
        .id()
}
