use std::time::Duration;

use bevy::prelude::*;

use crate::enemy::Health;

#[derive(Clone, Copy)]
pub enum DamageType {
    Physical,
    Magical,
}

#[derive(Component)]
pub struct Attack {
    damage: f32,
    interval: f32, // milliseconds
    pub timer: Timer,
    damage_type: DamageType,
}

#[derive(Component)]
pub struct Dead {
    killer: Entity,
}

impl Attack {
    pub fn to_damage(&self, source: Entity) -> Damage {
        Damage {
            damage_type: self.damage_type,
            amount: self.damage,
            source,
        }
    }
}

#[derive(Component)]
pub struct Damage {
    amount: f32,
    damage_type: DamageType,
    source: Entity,
}

impl Default for Attack {
    fn default() -> Self {
        Attack {
            damage: 10.5,
            interval: 1000.0,
            timer: Timer::new(Duration::from_millis(1000), TimerMode::Once),
            damage_type: DamageType::Physical,
        }
    }
}

pub fn apply_damage(mut commands: Commands, mut q_enemies: Query<(Entity, &mut Health, &Damage)>) {
    for (entity, mut health, damage) in q_enemies.iter_mut() {
        let new_health = health.current - damage.amount;
        if new_health <= 0.0 {
            commands.entity(entity).insert(Dead {
                killer: damage.source,
            });
        } else {
            health.current = new_health;
        }
        commands.entity(entity).remove::<Damage>();
    }
}
