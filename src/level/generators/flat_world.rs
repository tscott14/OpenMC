/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * June 3, 2024
 *
 * A flat world generator.
 */

use crate::{
    block::block::Block,
    level::{chunk::config::ChunkConfigs, data::ChunkBlockData, LevelPosition},
};

use super::WorldGenerator;

pub struct FlatWorldGenerator {
    pub seed: u64,
}

impl WorldGenerator for FlatWorldGenerator {
    fn generate_chunk(&mut self, configs: ChunkConfigs, position: LevelPosition) -> ChunkBlockData {
        ChunkBlockData::from_filled(&Block::Stone)
    }
}
