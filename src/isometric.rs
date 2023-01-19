use bevy::prelude::Vec2;

use crate::config::TILE_WIDTH;

#[derive(Clone, Copy, Debug)]
pub struct Coordinates(pub usize, pub usize);

pub type ScreenCoordinates = Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpriteDirection {
    NE,
    SE,
    SW,
    NW,
}

impl SpriteDirection {
    pub fn get_direction(origin: Coordinates, target: Coordinates) -> Self {
        let Coordinates(o_x, o_y) = origin;
        let Coordinates(t_x, t_y) = target;
        let x_diff: isize = t_x as isize - o_x as isize;
        let y_diff: isize = t_y as isize - o_y as isize;
        if x_diff > 0 {
            return SpriteDirection::SE;
        };
        if x_diff < 0 {
            return SpriteDirection::NW;
        };
        if y_diff > 0 {
            return SpriteDirection::SW;
        };
        if y_diff < 0 {
            return SpriteDirection::NE;
        };
        panic!();
    }

    pub fn flip_x(self) -> bool {
        return self == SpriteDirection::NW || self == SpriteDirection::SW;
    }

    pub fn facing_backwards(self) -> bool {
        return self == SpriteDirection::NW || self == SpriteDirection::NE;
    }
}

// https://clintbellanger.net/articles/isometric_math/
pub fn coordinates_to_screen(coordinates: &Coordinates) -> ScreenCoordinates {
    let Coordinates(x, y) = coordinates;
    let screen_x = (x - y) as f32 * (TILE_WIDTH / 2.0);
    let screen_y = (x + y) as f32 * (TILE_WIDTH / 4.0) * -1.0;
    ScreenCoordinates::new(screen_x, screen_y)
}
