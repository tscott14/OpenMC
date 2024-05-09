use crate::level::LevelPosition;
use super::{chunk::config::ChunkConfigs, data::ChunkBlockData};

pub trait WorldGenerator {
    fn generate_chunk(&mut self, configs: ChunkConfigs, position: LevelPosition) -> ChunkBlockData;
}

pub mod flat_world;

