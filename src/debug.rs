use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::isometric::ScreenCoordinates;

pub fn debug_polygon(debug_lines: &mut DebugLines, polygon: &Vec<ScreenCoordinates>) {
    let mut polygon = polygon.clone();
    polygon.push(*polygon.first().unwrap());
    for window in polygon.windows(2).into_iter() {
        let [first, second] = window else {
            panic!();
        };

        debug_lines.line(
            Vec3::new(first.x, first.y, 0.0),
            Vec3::new(second.x, second.y, 0.0),
            0.0,
        );
    }
}

pub fn debug_point(debug_lines: &mut DebugLines, point: &ScreenCoordinates) {
    debug_lines.line(
        Vec3::new(-90000.0, point.y, 0.0),
        Vec3::new(9000.0, point.y, 0.0),
        0.0,
    );
    debug_lines.line(
        Vec3::new(point.x, -90000.0, 0.0),
        Vec3::new(point.x, 9000.0, 0.0),
        0.0,
    );
}
