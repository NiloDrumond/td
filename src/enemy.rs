use bevy::{prelude::*, sprite::Anchor};

use crate::{
    assets::SpriteSheets,
    config::{SysLabel, ENEMY_OFFSET},
    isometric::ScreenCoordinates,
    map::CurrentMap,
    math::move_towards,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(test_spawn.after(SysLabel::LoadMap))
            .add_system(move_enemies)
            .add_system(check_enemies_through.after(move_enemies));
    }
}

#[derive(Component)]
pub struct Enemy {
    speed: f32,
    sprite_index: usize,
    last_waypoint: usize,
}

fn move_enemies(
    mut enemies_query: Query<(&mut Transform, &mut Enemy, &mut TextureAtlasSprite)>,
    map: Res<CurrentMap>,
    time: Res<Time>,
) {
    let waypoints = &map.path.waypoints;
    let _dt = time.delta().as_secs_f32();
    for (mut transform, mut enemy, mut sprite) in enemies_query.iter_mut() {
        let next_waypoint = &waypoints[enemy.last_waypoint + 1];
        let next_position = Vec3::new(
            next_waypoint.screen_coordinates.x,
            next_waypoint.screen_coordinates.y + ENEMY_OFFSET,
            transform.translation.z,
        );
        transform.translation = move_towards(transform.translation, next_position, enemy.speed);
        if transform.translation == next_position {
            let flip_x = next_waypoint.direction.flip_x();
            let facing_backwards = next_waypoint.direction.facing_backwards();
            let sprite_index = if facing_backwards {
                enemy.sprite_index + 4
            } else {
                enemy.sprite_index
            };
            sprite.flip_x = flip_x;
            sprite.index = sprite_index;
            enemy.last_waypoint += 1;
        }
    }
}


fn check_enemies_through(
    mut commands: Commands,
    enemies_query: Query<(Entity, &Enemy)>,
    map: Res<CurrentMap>,
) {
    let last_waypoint = &map.path.waypoints.len() - 1;
    for (entity, enemy) in enemies_query.iter() {
        if enemy.last_waypoint == last_waypoint {
            commands.entity(entity).despawn();
        }
    }
}

fn test_spawn(commands: Commands, handles: Res<SpriteSheets>, map: Res<CurrentMap>) {
    let enemy_sprites = &handles.enemy_sprites;
    spawn_enemy(commands, 32, enemy_sprites, map);
}

fn spawn_enemy(
    mut commands: Commands,
    sprite_index: usize,
    texture_atlas: &Handle<TextureAtlas>,
    current_map: Res<CurrentMap>,
) -> Entity {
    let start = current_map.path.waypoints.first().unwrap();
    let ScreenCoordinates {
        x: screen_x,
        y: screen_y,
    } = start.screen_coordinates;
    let flip_x = start.direction.flip_x();
    let facing_backwards = start.direction.facing_backwards();
    let sprite_index = if facing_backwards {
        sprite_index + 4
    } else {
        sprite_index
    };

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                anchor: Anchor::Center,
                index: sprite_index,
                flip_x,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(screen_x, screen_y + ENEMY_OFFSET, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Enemy {
            sprite_index,
            speed: 0.1,
            last_waypoint: 0,
        })
        .id()
}
