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

use crate::{
    level::{chunk::config::ChunkConfigs, mesh::culler::ChunkCacheBuffer},
    utils::position::{
        self,
        position::{ChunkPosition, GlobalPosition},
    },
};

use super::{
    chunk,
    data::ChunkBlockData,
    generators::{simple::SimpleTerrainGenerator, TerrainGenerators, TerrainServer},
    machine::ChunkStateNode, server::ChunkServer,
};

#[derive(Resource)]
pub struct WorldOrigin(GlobalPosition);

#[derive(Resource)]
pub struct RegionMaterial(Handle<StandardMaterial>);

impl RegionMaterial {
    pub fn new(
        asset_server: &mut AssetServer,
        material_server: &mut Assets<StandardMaterial>,
    ) -> Self {
        let stone_texture: Handle<Image> = asset_server.load("textures/blocks.png");
        let block_material = StandardMaterial {
            base_color_texture: Some(stone_texture.clone()),
            reflectance: 0f32,
            // cull_mode: None,
            ..default()
        };

        Self(material_server.add(block_material))
    }
}

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
        commands.insert_resource(WorldOrigin(GlobalPosition::from((0, 0, 0))));

        // Create the terrain server.
        commands.insert_resource(TerrainServer::simple_terrain(0));

        // Create the chunk spawn queue.
        commands.insert_resource(ChunkServer(vec![]));
    }

    pub fn populate_chunk_spawn_queue(
        mut terrain_server: ResMut<TerrainServer>,
        mut chunk_server: ResMut<ChunkServer>,
        camera: Query<&Transform, With<crate::camera::FirstPersonCamera>>,
    ) {
        let camera_pos = camera.single().translation;
        terrain_server.set_focus(camera_pos);

        // if the chunk requested is already present in the list, don't add it again.
        terrain_server.iter().for_each(|position| {
            if chunk_server
                .0
                .iter()
                .find(|state| state.position() == position)
                .is_some()
            {
                return;
            }

            chunk_server.0.push(ChunkState::SpawnRequested(position));
        });
    }

    pub fn process_chunk_build_chain(mut chunk_server: ResMut<ChunkServer>) {
        chunk_server.0.iter_mut().map(|state| {
            state.
            state = match state {
                ChunkStateNode::InitialSpawnRequestState(_) => ChunkStateNode::GenerateDataState(&state),
                ChunkStateNode::GenerateDataState(_) => ChunkStateNode::RefreshCacheState(&state),
                ChunkStateNode::RefreshCacheState(_) => ChunkStateNode::GenerateMeshState(&state),
                ChunkStateNode::GenerateMeshState(_) => ChunkStateNode::ActiveState(&state),
                ChunkStateNode::DespawnRequestedState(_) => ChunkStateNode::DespawnRequestedState(&state),
                _ => state,
            }
        });
    }

    // TODO: Generate SpawnRequested states.
    pub fn generate_spawn_requested_positions(
        mut terrain_server: ResMut<TerrainServer>,
        mut chunk_server: ResMut<ChunkServer>,
    ) {
        for chunk_position in terrain_server.iter() {
            chunk_server
                .0
                .push(ChunkState::SpawnRequested(chunk_position));
        }
    }

    // TODO: Method to convert SpawnRequested state to GeneratedData state.
    pub fn spawn_requested_into_generated_data(
        mut commands: Commands,
        mut terrain_server: ResMut<TerrainServer>,
        mut chunk_server: ResMut<ChunkServer>,
    ) {
        // Loop through all the server chunks and turn them into generated data chunks.
        chunk_server
            .iter_mut()
            .filter_map(|state| match state {
                ChunkStateNode::SpawnRequested(position) => {
                    Some(ChunkStateNode::GeneratedData(*position))
                }
                _ => None,
            })
            .map(|state| {
                let mut next: GenerateDataStateStruct = state.into();
                next.init(&commands, &terrain_server);
                next.exec();
            })
    }

    // TODO: Method to convert GeneratedData state to NeighborCacheRefresh state.
    pub fn generated_data_into_neighbor_cache_refresh() {}

    // TODO: Method to convert NeighborCacheRefresh state to SelfCullCacheRefresh state.
    pub fn neighbor_cache_refresh_into_self_cull_cache_refresh() {}

    // TODO: Method to convert SelfCullCacheRefresh state to MeshBufferRefresh state.
    pub fn self_cull_cache_refresh_into_mesh_buffer_refresh() {}

    // TODO: Method to convert MeshBufferRefresh state to actual state.
    pub fn mesh_buffer_refresh_into_active() {}

    // TODO: Method to convert active state to NeighborCacheRefresh
    pub fn active_into_neighbor_cache_refresh() {}

    // TODO: Method to convert active state to despawn state.
    pub fn active_into_despawn() {}

    fn spawn_chunk(
        commands: &mut Commands,
        terrain_server: &mut TerrainServer,
        mesh_server: &mut Assets<Mesh>,
        material: &mut RegionMaterial,
        position: ChunkPosition,
    ) -> ChunkState {
        log::info!("Spawn chunk {:?}", <(i32, i32, i32)>::from(position));
        let (x, y, z) = <(i32, i32, i32)>::from(GlobalPosition::from(position));

        let pbr_bundle = PbrBundle {
            mesh: mesh_server.add(Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            )),
            material: material.0.clone(),
            transform: Transform::from_xyz(x as f32, y as f32, z as f32),
            ..default()
        };

        // Generate chunk configuration.
        let config = ChunkConfigs {
            generation_req: true,
            mesh_cache_refresh_req: true,
            mesh_data_refresh_req: true,
            dealloc_req: false,
        };

        // Generate chunk data.
        let chunk_blocks = terrain_server.generate_chunk(position);

        // Add the chunk entity to the world.
        commands.spawn((
            pbr_bundle,
            position,
            config,
            chunk_blocks,
            ChunkCacheBuffer::default(), // This will be generated later.
        ));

        ChunkState::Active(position)
    }

    pub fn spawn_update(
        mut commands: Commands,
        mut terrain_server: ResMut<TerrainServer>,
        mut chunk_server: ResMut<ChunkServer>,
        mut mesh_server: ResMut<Assets<Mesh>>,
        mut material: ResMut<RegionMaterial>,
    ) {
        chunk_server.0 = chunk_server
            .0
            .iter()
            .map(|state| match state {
                ChunkState::SpawnRequested(position) => Self::spawn_chunk(
                    &mut commands,
                    &mut terrain_server,
                    &mut mesh_server,
                    &mut material,
                    *position,
                ),
                ChunkState::DespawnRequested(pos) => ChunkState::DespawnRequested(*pos),
                ChunkState::CullCacheRequested(pos) => ChunkState::CullCacheRequested(*pos),
                ChunkState::Active(pos) => ChunkState::Active(*pos),
            })
            .collect();
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

    pub fn apply_culling_status(
        mut chunks: Query<(
            &ChunkPosition,
            &mut ChunkConfigs,
            &ChunkBlockData,
            &mut ChunkCacheBuffer,
        )>,
    ) {
    }
}
