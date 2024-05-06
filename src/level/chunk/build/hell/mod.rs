pub struct HellServer;

impl super::prototype::WorldType for HellServer {
    fn get_chunk(
        &self,
        chunk_pos: crate::utils::position::chunk::ChunkPosition,
    ) -> crate::level::buffer::ChunkBuffer<crate::block::blocks::Block> {
        // TODO:
    }
}
