/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * May 9, 2024
 *
 * A configuration class for chunks.
 */
 
use std::fmt::Debug;

use bevy::{
    asset::*,
    ecs::{component::Component, system::ResMut},
    math::Vec3,
    render::{mesh::*, render_asset::*},
};

pub const CHUNK_SIZE: usize = 16;

#[derive(Component, Clone, Copy, Debug)]
pub struct ChunkConfigs {
    /// The chunk is void and needs generation.
    pub generation_req: bool, 

    /// The mesh cache needs to be regenerated.
    pub mesh_cache_refresh_req: bool,

    /// The mesh buffers need to be regenerated.
    pub mesh_data_refresh_req: bool,

    /// Request to be despawned
    pub dealloc_req: bool,
}

impl Default for ChunkConfigs {
    fn default() -> Self {
        Self {
            generation_req: true,
            mesh_cache_refresh_req: true,
            mesh_data_refresh_req: true,
            dealloc_req: false,
        }
    }
}
