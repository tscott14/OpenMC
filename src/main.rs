/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * May 8, 2024
 *
 * A Minecraft-like 3D voxel renderer programmed in Rust/Bevy.
 */

#![allow(dead_code, unused)]

use bevy::{
    pbr::wireframe::*,
    prelude::*,
    render::{settings::*, *},
    window::*,
    *,
};

mod block;
mod camera;
mod level;

use camera::FirstPersonCamera;
use level::{level::Level, mesh::mesh::ChunkMeshes};

fn main() {
    App::new()
        .add_plugins(OpenMCDefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, Level::setup)
        .add_systems(
            Update,
            (
                FirstPersonCamera::camera_movement,
                FirstPersonCamera::camera_rotation,
            ),
        )
        .add_systems(
            Update,
            (
                Level::spawn_update,
                Level::despawn_update,
                ChunkMeshes::refresh,
                level::mesh::culler::refresh_cache,
            ),
        )
        .add_systems(Update, developement_update)
        .run();
}

struct OpenMCDefaultPlugins;
impl Plugin for OpenMCDefaultPlugins {
    fn build(&self, app: &mut App) {
        let defaults = DefaultPlugins
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "OpenMC".into(),
                    cursor: Cursor {
                        grab_mode: CursorGrabMode::Confined,
                        visible: false,
                        ..default()
                    },
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
            })
            .add_systems(Update, toggle_cursor_capture)
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

fn toggle_cursor_capture(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Some(mut window) = window.get_single_mut().ok() {
        match window.cursor.grab_mode {
            CursorGrabMode::None => {
                if keyboard_input.just_pressed(KeyCode::KeyC) {
                    window.cursor.grab_mode = CursorGrabMode::Confined;
                    window.cursor.visible = false;
                    info!("Cursor locked");
                }
            }
            CursorGrabMode::Confined | CursorGrabMode::Locked => {
                if keyboard_input.just_pressed(KeyCode::KeyC) {
                    window.cursor.grab_mode = CursorGrabMode::None;
                    window.cursor.visible = true;
                    info!("Cursor unlocked");
                }
            }
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(FirstPersonCamera::create_bundle(0.0, 20.0, 2.0));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::CLEAR_SUNRISE,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_vec4(Vec4::new(1.0, 1.0, 1.0, 0.0)),
            ..default()
        },
        ..default()
    });
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
