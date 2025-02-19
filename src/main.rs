/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * May 8, 2024
 *
 * A Minecraft-like 3D voxel renderer programmed in Rust/Bevy.
 */

// #![allow(dead_code, unused)]

use bevy::{
    asset::RenderAssetUsages, dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin}, pbr::wireframe::*, prelude::*, render::mesh::{Indices, PrimitiveTopology}, *
};
use block::block::Block;
use camera_controller::{CameraController, CameraControllerPlugin};
use chunk::{chunk::{IndexableChunkList, CHUNK_SIZE}, mesher::ChunkMesh};
// use camera::FirstPersonCamera;

mod camera_controller;
mod chunk;
mod utils;
mod block;

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
            FpsOverlayPlugin {
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
            },
        );

        app.add_plugins(CameraControllerPlugin);
    }
}

fn main() {
    App::new()
        .add_plugins(CraftoriaDefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, developement_update)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    
) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    let custom_texture_handle: Handle<Image> = asset_server.load("textures/classic.png");

    let mut list = IndexableChunkList {
        data: Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
    };

    for y in 0..CHUNK_SIZE as isize{
        for x in 0..CHUNK_SIZE as isize {
            for z in 0..CHUNK_SIZE as isize {
                let eq = x * x + z * z - y* y;
                let thres = CHUNK_SIZE as isize;
                if  eq < thres && eq > -thres{
                    list[(x as usize, y as usize, z as usize)] = Block::Grass;

                }
            }
        }
    }

    let chunk_mesh_data = ChunkMesh::generate_mesh(&list);

    let chunk_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION, 
        chunk_mesh_data.verticies
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        chunk_mesh_data.normals
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        chunk_mesh_data.uvs
    )
    .with_inserted_indices(
        Indices::U32(chunk_mesh_data.indices),
    );

    let cube_mesh_handle: Handle<Mesh> = meshes.add(chunk_mesh);

    commands.spawn((
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(custom_texture_handle),
            // base_color: Color::srgb_u8(64, 255, 127),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0)
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

// https://bevyengine.org/learn/migration-guides/0-13-to-0-14/#remove-close-on-esc
pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

// // https://bevy-cheatbook.github.io/window/mouse-grab.html
// fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
//     let mut primary_window = q_windows.single_mut();

//     // if you want to use the cursor, but not let it leave the window,
//     // use `Confined` mode:
//     primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;

//     // for a game that doesn't use the cursor (like a shooter):
//     // use `Locked` mode to keep the cursor in one place
//     primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;

//     // also hide the cursor
//     primary_window.cursor_options.visible = false;
// }

// // https://bevy-cheatbook.github.io/window/mouse-grab.html
// fn cursor_ungrab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
//     let mut primary_window = q_windows.single_mut();

//     primary_window.cursor_options.grab_mode = CursorGrabMode::None;
//     primary_window.cursor_options.visible = true;
// }
