use std::ops::AddAssign;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin, PixelProjection};

use crate::{
    config::{MIN_CAMERA_HEIGHT, MIN_CAMERA_WIDTH, PAN_BUTTON, ZOOM_STEP},
    map::CurrentMap,
};

#[derive(Component)]
struct MainCamera;

pub struct CameraPlugin;

#[derive(Resource, Default)]
pub struct MouseWorldPos {
    pub x: f32,
    pub y: f32,
}

impl Into<Vec2> for MouseWorldPos {
    fn into(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(MouseWorldPos::default())
            .add_plugin(PixelCameraPlugin)
            // .add_plugin(PixelBorderPlugin {
            //     color: Color::rgb(0.1, 0.1, 0.1),
            // })
            .add_startup_system(setup_camera)
            .add_startup_system_to_stage(StartupStage::PostStartup, offset_camera)
            .add_system(pan_camera)
            .add_system(track_mouse_world_position);
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(PixelCameraBundle::from_resolution(320, 240))
        .insert(MainCamera);
}

fn pan_camera(
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut camera_query: Query<(&mut Transform, &mut PixelProjection)>,
) {
    let mut pan = Vec2::ZERO;
    let mut scroll = 0.0;
    for ev in ev_motion.iter() {
        pan += ev.delta;
    }

    for ev in ev_scroll.iter() {
        scroll = ev.y;
    }

    let (mut camera_transform, mut projection) = camera_query.single_mut();

    if input_mouse.pressed(PAN_BUTTON) && pan.length_squared() > 0.0 {
        let x = camera_transform.translation.x - pan.x;
        let y = camera_transform.translation.y + pan.y;
        camera_transform.translation = Vec3::new(x, y, camera_transform.translation.z);
    }
    if scroll.abs() != 0.0 {
        if let Some(desired_width) = projection.desired_width {
            projection.desired_width = Some(i32::max(
                desired_width - scroll as i32 * ZOOM_STEP,
                MIN_CAMERA_WIDTH,
            ));
        }
        if let Some(desired_height) = projection.desired_height {
            projection.desired_height = Some(i32::max(
                desired_height - scroll as i32 * ZOOM_STEP,
                MIN_CAMERA_HEIGHT,
            ));
        }
    }
}

fn track_mouse_world_position(
    // need to get window dimensions
    windows: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_pos: ResMut<MouseWorldPos>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = windows.get_primary().unwrap();
    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        let Vec2 { x, y } = world_pos;
        *mouse_pos = MouseWorldPos { x, y };
    }
}

fn offset_camera(
    mut q_camera: Query<&mut Transform, With<PixelProjection>>,
    current_map: Res<CurrentMap>,
) {
    let mut transform = q_camera.single_mut();
    transform
        .translation
        .add_assign(Vec3::new(0.0, current_map.height_offset, 0.0));
}
