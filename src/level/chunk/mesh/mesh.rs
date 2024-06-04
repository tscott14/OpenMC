/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * June 3, 2024
 *
 * A Utility class to generate mesh data for rendering.
 */

use std::fmt::Debug;

use bevy::{
    asset::{Assets, Handle},
    ecs::{
        query,
        system::{Query, ResMut},
    },
    render::{
        mesh::{self, Mesh, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};

use crate::{
    block::{block::Block, block_utils::BlockFaceCodes},
    level::{
        buffer::ChunkBuffer,
        chunk::config::{ChunkConfigs, CHUNK_SIZE},
        data::ChunkBlockData,
        mesh::culler::BlockFaceRenderConfig,
        LevelPosition,
    },
};

use super::culler::ChunkCacheBuffer;

pub struct ChunkMeshBuffer(Vec<([([f32; 3], [f32; 2], [f32; 3]); 4])>);

impl ChunkMeshBuffer {
    pub fn get(&self) -> &Vec<([([f32; 3], [f32; 2], [f32; 3]); 4])> {
        &self.0
    }

    pub fn vertex_buffer(&self) -> Vec<[f32; 3]> {
        self.0.iter().flatten().map(|vertex| vertex.0).collect()
    }

    pub fn uv_buffer(&self) -> Vec<[f32; 2]> {
        self.0.iter().flatten().map(|vertex| vertex.1).collect()
    }

    pub fn normal_buffer(&self) -> Vec<[f32; 3]> {
        self.0.iter().flatten().map(|vertex| vertex.2).collect()
    }

    pub fn face_buffer(&self) -> Vec<u32> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let i = i as u32 * 4;
                [i, i + 1, i + 2, i + 2, i + 1, i + 3]
            })
            .flatten()
            .collect()
    }

    pub fn populate(block_data: &ChunkBlockData, cache_buffer: &ChunkCacheBuffer) -> Self {
        let mut buffer = Vec::new();
        for ((position, _)) in ChunkBuffer::<()>::new().iter() {
            let block = block_data.get(position);
            let config = cache_buffer.get(position);

            for code in BlockFaceCodes::variants() {
                let (x, y, z) = position.get_xyz();

                if !config.is_with(code) {
                    continue;
                }

                let vertex_data = code
                    .vertex_data()
                    .map(|v| [v[0] + x as f32, v[1] + y as f32, v[2] + z as f32]);
                let uv_data = code.texture_data().map(|uv| {
                    let (u, v) = block.atlas_position().unwrap_or((3, 3));
                    [(uv[0] + u as f32) / 4.0, (uv[1] + v as f32) / 4.0]
                });
                let normal_data = code.normal_data();

                buffer.extend([[
                    (vertex_data[0], uv_data[0], normal_data),
                    (vertex_data[1], uv_data[1], normal_data),
                    (vertex_data[2], uv_data[2], normal_data),
                    (vertex_data[3], uv_data[3], normal_data),
                ]]);
            }
        }

        Self(buffer)
    }
}

impl Debug for ChunkMeshBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ChunkMeshBuffer:");
        writeln!(f, "Vertex Buffer: {:?}", self.vertex_buffer());
        writeln!(f, "UV Buffer: {:?}", self.uv_buffer());
        writeln!(f, "Normal Buffer: {:?}", self.normal_buffer());
        writeln!(f, "Face Buffer: {:?}", self.face_buffer());
        Ok(())
    }
}

pub struct ChunkMeshes;

impl ChunkMeshes {
    pub fn refresh(
        mut query: Query<(
            &Handle<Mesh>,
            &mut ChunkConfigs,
            &mut ChunkCacheBuffer,
            &ChunkBlockData,
        )>,
        mut mesh_server: ResMut<Assets<Mesh>>,
    ) {
        query
            .iter_mut()
            .for_each(|(handle, mut config, cache_buffer, block_data)| {
                // The cache does NOT need to be regenerated.
                if !config.mesh_data_refresh_req {
                    return;
                }

                // The chunks cache needs to be generated first.
                if config.mesh_cache_refresh_req {
                    return;
                }

                // println!("cahce_buffer: {:?}", cache_buffer);
                let mesh_buffer = ChunkMeshBuffer::populate(&block_data, &cache_buffer);

                mesh_server.insert(
                    handle.clone(),
                    Mesh::new(
                        PrimitiveTopology::TriangleList,
                        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                    )
                    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_buffer.vertex_buffer())
                    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_buffer.normal_buffer())
                    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_buffer.uv_buffer())
                    .with_inserted_indices(mesh::Indices::U32(mesh_buffer.face_buffer())),
                );

                config.mesh_data_refresh_req = true;
            });
    }
}
