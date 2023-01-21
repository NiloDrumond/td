use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::{
    debug::{debug_point, debug_polygon},
    enemy::Enemy,
    isometric::ScreenCoordinates,
    math::point_inside_polygon,
};

use super::Tower;

pub enum TargetingMode {
    Nearest,
    First,
    Last,
}

impl Default for TargetingMode {
    fn default() -> Self {
        TargetingMode::First
    }
}

#[derive(Component, Default)]
pub struct Targeting {
    pub mode: TargetingMode,
    pub range_polygon: Vec<ScreenCoordinates>,
}

#[derive(Component, Debug)]
pub struct Target {
    pub enemy: Entity,
}

// TODO: different targeting modes
fn select_target(
    mode: &TargetingMode,
    enemies: Vec<Entity>,
    q_enemies: &Query<(Entity, &Transform), With<Enemy>>,
) -> Option<Entity> {
    if let Some(first) = enemies.first() {
        return Some(*first);
    }
    return None;
}

pub fn assign_targets(
    mut commands: Commands,
    mut q_towers: Query<(Entity, &Tower, &Targeting, Option<&mut Target>)>,
    q_enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut debug_lines: ResMut<DebugLines>,
) {
    for (tower_entity, tower, targeting, prev_target) in q_towers.iter_mut() {
        debug_polygon(&mut debug_lines, &targeting.range_polygon);
        let mut enemies_in_range: Vec<Entity> = vec![];

        for (enemy_entity, transform) in q_enemies.iter() {
            let point = ScreenCoordinates::new(transform.translation.x, transform.translation.y);
            debug_point(&mut debug_lines, &point);
            if point_inside_polygon(point, &targeting.range_polygon) {
                enemies_in_range.push(enemy_entity);
            }
        }
        let new_target = select_target(&targeting.mode, enemies_in_range, &q_enemies);
        let Some(new_target) = new_target else {
            if let Some(_prev_target) = prev_target {
                commands.entity(tower_entity).remove::<Target>();
            }
            continue;
        };

        if let Some(mut prev_target) = prev_target {
            if prev_target.enemy != new_target {
                prev_target.enemy = new_target;
            }
        } else {
            commands
                .get_entity(tower_entity)
                .unwrap()
                .insert(Target { enemy: new_target });
        }
    }
}
