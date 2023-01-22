use bevy::{prelude::*, sprite::Anchor, time::FixedTimestep};

use crate::{
    assets::SpriteSheets,
    camera::MouseWorldPos,
    config::{BASE_Z_INDEX, TILE_HEIGHT_OFFSET},
    isometric::{coordinates_to_screen, screen_to_coordinates, Coordinates, ScreenCoordinates},
};

use self::{
    damage::{apply_damage, Attack},
    targeting::{assign_targets, Target, Targeting},
};

pub use self::damage::Dead;

mod damage;
mod targeting;

pub struct TowerPlugin;

#[derive(Component)]
pub struct Tower {
    sprite_index: usize,
    position: Coordinates,
    range: usize,
}

#[derive(Bundle)]
struct TowerBundle {
    tower: Tower,
    targeting: Targeting,
    sprite: SpriteSheetBundle,
    attack: Attack,
}

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(test_spawn).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(60.0))
                .with_system(assign_targets)
                .with_system(tick_towers)
                .with_system(apply_damage),
        );
    }
}

fn test_spawn(
    commands: Commands,
    handles: Res<SpriteSheets>,
    mouse_pos: Res<MouseWorldPos>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_released(MouseButton::Left) {
        let tower_sprites = &handles.enemy_sprites;
        spawn_tower(
            commands,
            0,
            tower_sprites,
            screen_to_coordinates(&ScreenCoordinates {
                x: mouse_pos.x,
                y: mouse_pos.y - TILE_HEIGHT_OFFSET,
            }),
        );
    }
}

fn spawn_tower(
    mut commands: Commands,
    sprite_index: usize,
    texture_atlas: &Handle<TextureAtlas>,
    position: Coordinates,
) -> Entity {
    let ScreenCoordinates {
        x: screen_x,
        y: screen_y,
    } = coordinates_to_screen(&position);

    let range = 1;
    let range_polygon = position.get_vertices(range);

    commands
        .spawn(TowerBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas.clone(),
                sprite: TextureAtlasSprite {
                    anchor: Anchor::Center,
                    index: sprite_index,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        screen_x,
                        screen_y + TILE_HEIGHT_OFFSET,
                        -screen_y + BASE_Z_INDEX,
                    ),
                    ..default()
                },
                ..default()
            },
            tower: Tower {
                sprite_index,
                position,
                range,
            },
            targeting: Targeting {
                range_polygon,
                ..default()
            },
            attack: Attack::default(),
        })
        .id()
}

fn tick_towers(
    mut commands: Commands,
    mut q_towers: Query<(Entity, &mut Attack, Option<&Target>)>,
    time: Res<Time>,
) {
    for (entity, mut attack, target) in q_towers.iter_mut() {
        attack.timer.tick(time.delta());

        if attack.timer.finished() {
            if let Some(target) = target {
                let damage = attack.to_damage(entity);
                commands.entity(target.enemy).insert(damage);
                attack.timer.reset();
            }
        }
    }
}
