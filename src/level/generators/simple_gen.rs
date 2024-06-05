/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * June 3, 2024
 *
 * A flat world generator.
 */

use crate::{
    block::block::Block,
    level::{
        buffer::ChunkBuffer, chunk::config::ChunkConfigs, config::CHUNK_SIZE, data::ChunkBlockData,
        position, LevelPosition,
    },
};

use super::WorldGenerator;

pub struct SimpleWorldGenerator {
    pub seed: u64,
}

impl SimpleWorldGenerator {
    fn calc(
        &self,
        position: LevelPosition,
        config: &[((Option<i32>, Option<i32>), Block)],
    ) -> Block {
        for option in config {
            let lower_bound = option.0 .0.unwrap_or(std::i32::MIN);
            let upper_bound = option.0 .1.unwrap_or(std::i32::MAX);
            if position.y >= lower_bound && position.y < upper_bound {
                return option.1;
            }
        }
        return Block::Air;
    }
}

impl WorldGenerator for SimpleWorldGenerator {
    fn generate_chunk(&mut self, configs: ChunkConfigs, position: LevelPosition) -> ChunkBlockData {
        let config = [
            ((None, Some(1)), Block::Bedrock),
            ((Some(1), Some(CHUNK_SIZE as i32 - 4)), Block::Stone),
            (
                (Some(CHUNK_SIZE as i32 - 4), Some(CHUNK_SIZE as i32 - 1)),
                Block::Dirt,
            ),
            ((Some(CHUNK_SIZE as i32 - 1), None), Block::Grass),
        ];

        ChunkBlockData::from_fn(position, |world, _local| self.calc(world, &config))
    }
}
