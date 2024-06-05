/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * June 3, 2024
 *
 * A Utility class to generate optomized block data for rendering.
 */

use bevy::ecs::{component::Component, system::Query};

use crate::{
    block::{self, block::Block, block_utils::BlockFaceCodes},
    level::{
        buffer::ChunkBuffer, chunk::config::ChunkConfigs, data::ChunkBlockData, LevelPosition,
    },
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BlockFaceRenderConfig(u8);

impl BlockFaceRenderConfig {
    pub fn from_u8(code: u8) -> Self {
        Self(code)
    }

    pub fn with(self, block_face: crate::block::block_utils::BlockFaceCodes) -> Self {
        Self(self.0 | block_face.mask_u8())
    }

    pub fn is_with(&self, block_face: crate::block::block_utils::BlockFaceCodes) -> bool {
        self.0 >> block_face.as_u8() & 1 == 1
    }

    pub fn empty() -> Self {
        Self::from_u8(0)
    }

    pub fn full() -> Self {
        use crate::block::block_utils::BlockFaceCodes;
        Self::empty()
            .with(BlockFaceCodes::BlockFacePositiveX)
            .with(BlockFaceCodes::BlockFaceNegativeX)
            .with(BlockFaceCodes::BlockFacePositiveY)
            .with(BlockFaceCodes::BlockFaceNegativeY)
            .with(BlockFaceCodes::BlockFacePositiveZ)
            .with(BlockFaceCodes::BlockFaceNegativeZ)
    }
}

impl Default for BlockFaceRenderConfig {
    fn default() -> Self {
        Self::full()
    }
}

#[derive(Component, Debug, Default)]
pub struct ChunkCacheBuffer(pub ChunkBuffer<BlockFaceRenderConfig>);

impl ChunkCacheBuffer {
    pub fn set(&mut self, pos: LevelPosition, config: &BlockFaceRenderConfig) {
        self.0.set(pos, config)
    }

    pub fn get(&self, pos: LevelPosition) -> &BlockFaceRenderConfig {
        self.0.get(pos)
    }

    fn config(
        &self,
        pos: crate::level::LevelPosition,
        block_data: &ChunkBlockData,
    ) -> BlockFaceRenderConfig {
        // let (x, y, z) = pos.get_xyz();
        let xp = pos.offset(1, 0, 0);
        let xn = pos.offset(-1, 0, 0);
        let yp = pos.offset(0, 1, 0);
        let yn = pos.offset(0, -1, 0);
        let zp = pos.offset(0, 0, 1);
        let zn = pos.offset(0, 0, -1);

        let bxp = block_data.try_get(xp).unwrap_or(&Block::Air);
        let bxn = block_data.try_get(xn).unwrap_or(&Block::Air);
        let byp = block_data.try_get(yp).unwrap_or(&Block::Air);
        let byn = block_data.try_get(yn).unwrap_or(&Block::Air);
        let bzp = block_data.try_get(zp).unwrap_or(&Block::Air);
        let bzn = block_data.try_get(zn).unwrap_or(&Block::Air);

        let config = 0
            | (bxp.transparent() as u8) << BlockFaceCodes::BlockFacePositiveX.as_u8()
            | (bxn.transparent() as u8) << BlockFaceCodes::BlockFaceNegativeX.as_u8()
            | (byp.transparent() as u8) << BlockFaceCodes::BlockFacePositiveY.as_u8()
            | (byn.transparent() as u8) << BlockFaceCodes::BlockFaceNegativeY.as_u8()
            | (bzp.transparent() as u8) << BlockFaceCodes::BlockFacePositiveZ.as_u8()
            | (bzn.transparent() as u8) << BlockFaceCodes::BlockFaceNegativeZ.as_u8();

            

        BlockFaceRenderConfig::from_u8(config)
    }

    pub fn populate(&mut self, block_data: &ChunkBlockData) {
        ChunkBuffer::<()>::default().iter().for_each(|(pos, _)| {
            self.set(pos, &self.config(pos, block_data));
        });
    }
}

pub fn refresh_cache(
    mut chunks: Query<(&mut ChunkConfigs, &ChunkBlockData, &mut ChunkCacheBuffer)>,
) {
    chunks
        .iter_mut()
        .filter(|(config, _, _)| config.mesh_cache_refresh_req)
        .for_each(|(mut config, block_data, mut cache_buffer)| {
            config.mesh_cache_refresh_req = false;
            config.mesh_data_refresh_req = true;
            
            cache_buffer.populate(&block_data);
        });
}
