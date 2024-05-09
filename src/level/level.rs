/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * June 3, 2024
 *
 * A class to handle everything involving level management.
 */

 use bevy::{
    log,
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
    utils::HashMap,
};

use crate::level::{
    chunk::config::ChunkConfigs, config::CHUNK_SIZE, generators::WorldGenerator,
    mesh::culler::ChunkCacheBuffer,
};

use super::{generators::flat_world::FlatWorldGenerator, LevelPosition};

#[derive(Resource)]
pub struct WorldOrigin(LevelPosition);

#[derive(Resource)]
pub struct RegionMaterial(Handle<StandardMaterial>);

impl RegionMaterial {
    pub fn new(
        asset_server: &mut AssetServer,
        material_server: &mut Assets<StandardMaterial>,
    ) -> Self {
        let stone_texture: Handle<Image> = asset_server.load("textures/stone.png");
        let block_material = StandardMaterial {
            base_color_texture: Some(stone_texture.clone()),
            reflectance: 0f32,
            cull_mode: None,
            ..default()
        };

        Self(material_server.add(block_material))
    }
}

#[derive(Resource)]
pub struct ChunkSpawnQueue(Vec<LevelPosition>);

#[derive(Resource)]
pub struct ChunkDespawnQueue(Vec<LevelPosition>);

pub struct Level;

impl Level {
    pub fn setup(
        mut commands: Commands,
        mut asset_server: Res<AssetServer>,
        mut mesh_server: ResMut<Assets<Mesh>>,
        mut material_server: ResMut<Assets<StandardMaterial>>,
    ) {
        // Create and add universal region material.
        commands.insert_resource(RegionMaterial::new(
            &mut asset_server.clone(),
            &mut material_server,
        ));

        // Create the world origin.
        commands.insert_resource(WorldOrigin(LevelPosition::new(0, 0, 0)));

        // Create the chunk spawn queue.
        commands.insert_resource(ChunkSpawnQueue(
            (0..10 * 10)
                .map(|i| {
                    let (x, z) = (i / 10 - 5, i % 10 - 5);
                    LevelPosition::from_chunk_xyz(x as i32, 0, z as i32)
                })
                .collect(),
        ));

        // Create the chunk despawn queue.
        commands.insert_resource(ChunkDespawnQueue(vec![]));
    }

    pub fn spawn_update(
        mut commands: Commands,
        mut chunk_spawn_queue: ResMut<ChunkSpawnQueue>,
        mut material: ResMut<RegionMaterial>,
        mut mesh_server: ResMut<Assets<Mesh>>,
    ) {
        let mut generator = FlatWorldGenerator { seed: 0 };

        for req_chunk_pos in chunk_spawn_queue.0.drain(..) {
            log::info!("Spawn chunk {:?}", req_chunk_pos.chunk_xyz());
            let (x, y, z) = req_chunk_pos.get_xyz();

            // Generate chunk configuration.
            let configs = ChunkConfigs {
                generation_req: true,
                mesh_cache_refresh_req: true,
                mesh_data_refresh_req: true,
                dealloc_req: false,
            };

            // generate the block data for the chunk -> BlockData.
            let block_data = generator.generate_chunk(configs, req_chunk_pos.clone());

            let pbr_bundle = PbrBundle {
                mesh: mesh_server.add(Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                )),
                material: material.0.clone(),
                transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                ..default()
            };

            commands.spawn((
                pbr_bundle,
                configs,
                block_data,
                ChunkCacheBuffer::default(), // This will be generated later.
            ));
        }
    }

    pub fn despawn_update(
        mut commands: Commands,
        mut query: Query<(Entity, &ChunkConfigs, &Transform)>,
    ) {
        query
            .iter()
            .filter(|(_, conf, _)| conf.dealloc_req)
            .for_each(|(entity, _, trans)| {
                log::info!("Despawn chunk {:?}", trans);
                commands.entity(entity).despawn()
            });
    }
}
