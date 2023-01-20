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

// NOTE: https://clintbellanger.net/articles/isometric_math/
pub fn coordinates_to_screen(coordinates: &Coordinates) -> ScreenCoordinates {
    let Coordinates(x, y) = coordinates;
    let x = *x as isize;
    let y = *y as isize;
    let screen_x = (x - y) as f32 * (TILE_WIDTH / 2.0);
    let screen_y = (x + y) as f32 * (TILE_WIDTH / 4.0) * -1.0;
    ScreenCoordinates::new(screen_x, screen_y)
}

// sx = (x - y) * half
// sx = half.x - half.y
// half.x = sx + half.y
// width.x = 2.sx + width.y
// x = (2.sx) / width + y
// x = (2.sx) / width - 4.sy / width - x
// x = (sx - 2sy) / width 
//
// sy = (x + y) * quarter * -1
// -sy = quarter.x + quarter.y
// quarter.y = -sy - quarter.x
// width.y = -4.sy - width.x
// y = -4.sy / width - x
// y = -4.sy / width - 2.sx / width - y
// y = (-2.sy - sx) / width

pub fn screen_to_coordinates(screen_coordinates: &ScreenCoordinates) -> Coordinates {
    let ScreenCoordinates { x, y } = screen_coordinates;
    let iso_x = (x - 2.0 * y) / TILE_WIDTH;
    let iso_y = -1.0 * (2.0 * y + x) / TILE_WIDTH;
    Coordinates(iso_x as usize, iso_y as usize)
}
