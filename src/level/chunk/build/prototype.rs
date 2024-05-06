pub trait WorldType {
    fn get_chunk(
        &self,
        position: crate::utils::position::chunk::ChunkPosition,
    ) -> crate::level::buffer::ChunkBuffer<crate::block::blocks::Block>;
}
