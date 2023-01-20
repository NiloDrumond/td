use bevy::{prelude::*, sprite::Anchor};

use crate::{
    assets::SpriteSheets,
    camera::MouseWorldPos,
    config::TILE_HEIGHT_OFFSET,
    isometric::{coordinates_to_screen, screen_to_coordinates, Coordinates, ScreenCoordinates},
    map::CurrentMap,
};

pub struct TowerPlugin;

#[derive(Component)]
pub struct Tower {
    sprite_index: usize,
}

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(test_spawn);
    }
}

fn test_spawn(
    commands: Commands,
    handles: Res<SpriteSheets>,
    mouse_pos: Res<MouseWorldPos>,
    buttons: Res<Input<MouseButton>>,
    current_map: Res<CurrentMap>,
) {
    if buttons.just_released(MouseButton::Left) {
        let height_offset = current_map.height_offset;

        let total_offset = height_offset + TILE_HEIGHT_OFFSET;
        let tower_sprites = &handles.enemy_sprites;
        spawn_tower(
            commands,
            32,
            tower_sprites,
            screen_to_coordinates(&ScreenCoordinates {
                x: mouse_pos.x,
                y: mouse_pos.y - total_offset,
            }),
            current_map,
        );
    }
}

fn spawn_tower(
    mut commands: Commands,
    sprite_index: usize,
    texture_atlas: &Handle<TextureAtlas>,
    position: Coordinates,
    current_map: Res<CurrentMap>,
) -> Entity {
    // let start = current_map.path.waypoints.first().unwrap();
    // let flip_x = start.direction.flip_x();
    // let facing_backwards = start.direction.facing_backwards();
    // let sprite_index = if facing_backwards {
    //     sprite_index + 4
    // } else {
    //     sprite_index
    // };

    let ScreenCoordinates {
        x: screen_x,
        y: screen_y,
    } = coordinates_to_screen(&position);

    let height_offset = current_map.height_offset;
    let total_offset = height_offset + TILE_HEIGHT_OFFSET;

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                anchor: Anchor::Center,
                index: sprite_index,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(screen_x, screen_y + total_offset, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Tower { sprite_index })
        .id()
}
