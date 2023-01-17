use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_pixel_camera::{
    PixelCameraBundle, PixelCameraPlugin, PixelProjection,
};

use crate::config::{MIN_CAMERA_HEIGHT, MIN_CAMERA_WIDTH, PAN_BUTTON, ZOOM_STEP};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(PixelCameraPlugin)
            // .add_plugin(PixelBorderPlugin {
            //     color: Color::rgb(0.1, 0.1, 0.1),
            // })
            .add_startup_system(setup_camera)
            .add_system(pan_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_resolution(320, 240));
    // commands.spawn(Camera2dBundle::default());
    // commands.spawn(Camera3dBundle {
    //     projection: OrthographicProjection {
    //         scale: 3.0,
    //         scaling_mode: ScalingMode::FixedVertical(2.0),
    //         ..default()
    //     }
    //     .into(),
    //     transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
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
