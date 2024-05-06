use crate::{block::blocks::*, level::{buffer::*, configs::*}};


pub struct DecorateServer {
    seed: Seed,
    water_level: WaterHeight,
}

impl DecorateServer {
    pub fn new(seed: Seed) -> Self {
        Self {
            seed,
            water_level: 0,
        }
    }

    pub fn solidify(&self, chunk_buffer: &mut ChunkBuffer<bool>) -> ChunkBuffer<Block> {
        // TODO: convert bool field into a buffer of stone.
    }

    pub fn flood(&self, chunk_buffer: &mut ChunkBuffer<Block>) -> ChunkBuffer<Block> {
        // TODO: fill in oceans.
    }

    pub fn surface(&self, chunk_buffer: &mut ChunkBuffer<Block>) -> ChunkBuffer<Block> {
        // TODO: decorate the surface layers.
    }

    pub fn caves(&self, chunk_buffer: &mut ChunkBuffer<Block>) -> ChunkBuffer<Block> {
        // TODO: decorate the cave layers.
    }
}
