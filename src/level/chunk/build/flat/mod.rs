pub mod flat;

pub struct FlatServer;

impl super::prototype::WorldType for FlatServer {
    fn get_chunk(
        &self,
        chunk_pos: crate::utils::position::chunk::ChunkPosition,
    ) -> crate::level::buffer::ChunkBuffer<crate::block::blocks::Block> {
        // TODO:
    }
}