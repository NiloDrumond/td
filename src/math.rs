use bevy::prelude::Vec3;

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
