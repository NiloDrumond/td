use bevy::{
    prelude::*,
    sprite::Anchor,
};

use crate::{
    assets::SpriteSheets,
    config::{STATBAR_WIDTH, TILE_HEIGHT},
    enemy::{Enemy, Health},
};

pub struct HUDPlugin;

#[derive(Component)]
struct Statbar;

#[derive(Component)]
struct StatbarFill;

#[derive(Bundle)]
struct HealthbarBundle {
    base: SpriteSheetBundle,
    statbar: Statbar,
}

#[derive(Component)]
pub struct HealthBar {
    pub bar: Entity,
}

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(attach_health_bar)
            .add_system(update_health_bar);
    }
}

fn attach_health_bar(
    mut commands: Commands,
    q_enemies: Query<(Entity, &Transform), (With<Enemy>, Without<HealthBar>)>,
    handles: Res<SpriteSheets>,
) {
    let texture_atlas = &handles.statbar_sprites;
    for (entity, transform) in q_enemies.iter() {
        let base_translation = Vec3::new(
            transform.translation.x + STATBAR_WIDTH / 2.0,
            transform.translation.y + TILE_HEIGHT / 2.0,
            3.0,
        );
        let fill_translation = Vec3::new(0.0, 0.0, 4.0);

        let bar_entity = commands
            .spawn(HealthbarBundle {
                base: SpriteSheetBundle {
                    texture_atlas: texture_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        anchor: Anchor::CenterRight,
                        index: 1,
                        ..default()
                    },
                    transform: Transform {
                        translation: base_translation,
                        ..default()
                    },
                    ..default()
                },
                statbar: Statbar,
            })
            .id();
        commands.entity(bar_entity).add_children(|parent| {
            parent
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        anchor: Anchor::CenterRight,
                        index: 0,
                        ..default()
                    },
                    transform: Transform {
                        translation: fill_translation,
                        scale: Vec3::new(0.0, 1.0, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(StatbarFill);
        });

        commands
            .entity(entity)
            .insert(HealthBar { bar: bar_entity });
    }
}

fn update_health_bar(
    q_enemies: Query<(&Transform, &Health, &HealthBar), (Without<Statbar>, Without<StatbarFill>)>,
    mut q_bars: Query<
        (&mut Transform, &mut Visibility, &Children),
        (With<Statbar>, Without<StatbarFill>),
    >,
    mut q_child: Query<&mut Transform, (Without<Statbar>, With<StatbarFill>)>,
) {
    for (transform, health, health_bar) in q_enemies.iter() {
        let (mut bar_transform, mut bar_visiblity, children) =
            q_bars.get_mut(health_bar.bar).unwrap();
        if health.current != health.max {
            bar_transform.translation = Vec3::new(
                transform.translation.x + STATBAR_WIDTH / 2.0,
                transform.translation.y + TILE_HEIGHT / 2.0,
                3.0,
            );
            bar_visiblity.is_visible = true;
            let fill = children.get(0).unwrap();
            let mut fill = q_child.get_mut(*fill).unwrap();
            fill.scale.x = (health.max - health.current) / health.max;
        } else {
            bar_visiblity.is_visible = false;
        }
    }
}
