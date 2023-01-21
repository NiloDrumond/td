use bevy::prelude::Vec3;
use combination::combine;

use crate::isometric::ScreenCoordinates;

pub fn magnitude(v: &Vec3) -> f32 {
    let squared = |x| f32::powi(x, 2);
    f32::sqrt(squared(v.x) + squared(v.y) + squared(v.z))
}

pub fn move_towards(origin: Vec3, target: Vec3, max_distance: f32) -> Vec3 {
    let diff = target - origin;
    let magnitude = magnitude(&diff);
    if magnitude <= max_distance || magnitude == 0.0 {
        return target;
    }
    origin + diff.normalize() * max_distance
}

// NOTE: https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
fn point_in_triangle(point: ScreenCoordinates, triangle: [ScreenCoordinates; 3]) -> bool {
    let p = point;
    let [p0, p1, p2] = triangle;
    let s = (p0.x - p2.x) * (p.y - p2.y) - (p0.y - p2.y) * (p.x - p2.x);
    let t = (p1.x - p0.x) * (p.y - p0.y) - (p1.y - p0.y) * (p.x - p0.x);

    if (s < 0.0) != (t < 0.0) && s != 0.0 && t != 0.0 {
        return false;
    }

    let d = (p2.x - p1.x) * (p.y - p1.y) - (p2.y - p1.y) * (p.x - p1.x);
    return d == 0.0 || (d < 0.0) == (s + t <= 0.0);
}

// NOTE: only works for concave polygons
pub fn point_inside_polygon(point: ScreenCoordinates, polygon: &Vec<ScreenCoordinates>) -> bool {
    let triangles = combine::from_vec_at(&polygon, 3);
    for triangle in triangles {
        let triangle: [ScreenCoordinates; 3] = triangle.try_into().unwrap();
        let inside = point_in_triangle(point, triangle);
        if inside {
            return true;
        }
    }
    false
}
