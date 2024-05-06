use crate::{block::blocks::Block, level::buffer::ChunkBuffer, utils::position::chunk::ChunkPosition};

use super::prototype::WorldType;

pub mod terrain;
pub mod decorate;

pub struct WorldServer;

impl WorldType for WorldServer {
    fn get_chunk(&self, chunk_pos: ChunkPosition) -> ChunkBuffer<Block> {
        ChunkBuffer::new()
    }
}