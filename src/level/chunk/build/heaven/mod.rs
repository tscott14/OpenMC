pub struct HeavenServer;

impl super::prototype::WorldType for HeavenServer {
    fn get_chunk(
        &self,
        chunk_pos: crate::utils::position::chunk::ChunkPosition,
    ) -> crate::level::buffer::ChunkBuffer<crate::block::blocks::Block> {
        // TODO:
    }
}