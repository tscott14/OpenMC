/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * May 9, 2024
 *
 * A wrapper class for camera.
 */

use bevy::{
    ecs::{component::*, event::*, query::*, system::*},
    input::{keyboard::*, mouse::*, *},
    math::*,
    prelude::*,
};

#[derive(Component)]
pub struct FirstPersonCamera;

impl FirstPersonCamera {
    pub fn create_bundle(x: f32, y: f32, z: f32) -> (Camera3dBundle, FirstPersonCamera) {
        (
            Camera3dBundle {
                transform: Transform::from_xyz(x, y, z).looking_at(Vec3::ZERO, Vec3::Y),
                projection: Projection::Perspective(PerspectiveProjection {
                    far: 10000.0,
                    ..default()
                }),
                ..default()
            },
            FirstPersonCamera,
        )
    }

    pub fn camera_movement(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut camera: Query<&mut Transform, With<FirstPersonCamera>>,
        time: Res<Time>,
    ) {
        if let Some(mut camera) = camera.get_single_mut().ok() {
            let mut direction = Vec3::ZERO;
            
            if keyboard_input.pressed(KeyCode::KeyW) {
                direction.z += 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyA) {
                direction.x -= 1.1;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                direction.z -= 1.1;
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                direction.x += 1.1;
            }
            if keyboard_input.pressed(KeyCode::Space) {
                direction.y += 1.1;
            }
            if keyboard_input.pressed(KeyCode::ShiftLeft) {
                direction.y -= 1.1;
            }

            camera.translation += 10.0 * direction.normalize_or_zero() * time.delta_secs();
        }
    }

    pub fn camera_rotation(
        buttons: Res<ButtonInput<MouseButton>>,
        mut motion_evr: EventReader<MouseMotion>,
        mut camera: Query<&mut Transform, With<FirstPersonCamera>>,
        time: Res<Time>,
    ) {
        let mut transform = camera.single_mut();

        // Thanks to sburris0 for providing a solution
        // that I was able to utilize to overcome the
        // inherent roll of the camera when rotating
        // the inbuilt quaternion property of bevy's
        // Transform component.
        // https://github.com/sburris0/bevy_flycam/blob/master/src/lib.rs
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        for event in motion_evr.read() {
            pitch -= (event.delta.y * time.delta_secs()).to_radians() * 10.0;
            yaw -= (event.delta.x * time.delta_secs()).to_radians() * 10.0;
        }

        pitch = pitch.clamp(-1.54, 1.54);

        // if buttons.pressed(MouseButton::Left) {
        let ry = Quat::from_axis_angle(Vec3::Y, yaw);
        let rx = Quat::from_axis_angle(Vec3::X, pitch);
        transform.rotation = ry * rx;
        // }
    }
}
