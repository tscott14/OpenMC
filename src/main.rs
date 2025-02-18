/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * May 8, 2024
 *
 * A Minecraft-like 3D voxel renderer programmed in Rust/Bevy.
 */

#![allow(dead_code, unused)]

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    input::{common_conditions::input_just_pressed, keyboard::Key},
    pbr::wireframe::*,
    prelude::*,
    render::{settings::*, *},
    window::*,
    *,
};
use camera_controller::{CameraController, CameraControllerPlugin};
// use camera::FirstPersonCamera;

mod camera_controller;
mod chunk;
mod utils;

struct CraftoriaDefaultPlugins;
impl Plugin for CraftoriaDefaultPlugins {
    fn build(&self, app: &mut App) {
        let defaults = DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Craftoria".into(),
                    ..default()
                }),
                ..default()
            });

        app.add_plugins((defaults, WireframePlugin))
            .insert_resource(WireframeConfig {
                global: true,
                default_color: Color::WHITE,
            })
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 0.5,
                ..default()
            });

        app.add_plugins(
            (FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        // Here we define size of our overlay
                        font_size: 42.0,
                        // If we want, we can use a custom font
                        font: default(),
                        // We could also disable font smoothing,
                        font_smoothing: text::FontSmoothing::default(),
                        ..default()
                    },
                    // We can also change color of the overlay
                    text_color: Color::srgb(1.0, 1.0, 1.0),
                    // We can also set the refresh interval for the FPS counter
                    // refresh_interval: core::time::Duration::from_millis(100),
                    enabled: true,
                    ..Default::default()
                },
            }),
        );

        app.add_plugins(CameraControllerPlugin);
    }
}

fn main() {
    App::new()
        .add_plugins(CraftoriaDefaultPlugins)
        // .add_systems(Update, bevy::input::system::exit_on_esc_system)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            cursor_grab.run_if(input_just_pressed(KeyCode::KeyG)),
        )
        .add_systems(
            PostUpdate,
            cursor_ungrab.run_if(input_just_pressed(KeyCode::KeyR)),
        )
        .add_systems(Update, developement_update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn(FirstPersonCamera::create_bundle(0.0, 20.0, 2.0));    
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController::default()
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn developement_update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<WireframeConfig>,
) {
    // Toggle wireframe rendering of all meshes
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        config.global = !config.global;
        match config.global {
            true => info!("Wireframe rendering enabled for all meshes"),
            false => info!("Wireframe rendering disabled for all meshes"),
        }
    }
}

// https://bevy-cheatbook.github.io/window/mouse-grab.html
fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    // if you want to use the cursor, but not let it leave the window,
    // use `Confined` mode:
    primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;

    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    primary_window.cursor_options.visible = false;
}

// https://bevy-cheatbook.github.io/window/mouse-grab.html
fn cursor_ungrab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor_options.grab_mode = CursorGrabMode::None;
    primary_window.cursor_options.visible = true;
}
