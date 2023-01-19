use bevy::prelude::Vec2;

use crate::config::TILE_WIDTH;

#[derive(Clone, Copy, Debug)]
pub struct Coordinates(pub usize, pub usize);


// https://clintbellanger.net/articles/isometric_math/
pub fn coordinates_to_screen(coordinates: &Coordinates) -> Vec2 {
    let Coordinates(x, y) = coordinates;
    let screen_x = (x - y) as f32 * (TILE_WIDTH / 2.0);
    let screen_y = (x + y) as f32 * (TILE_WIDTH / 4.0) * -1.0;
    Vec2::new(screen_x, screen_y)
}
