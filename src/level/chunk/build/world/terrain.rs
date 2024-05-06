use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d;

use crate::{
    level::{buffer::ChunkBuffer, configs::Seed},
    utils::position::chunk::ChunkPosition,
};

pub struct TerrainGenerator {
    seed: Seed,
}

impl TerrainGenerator {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn get(&self, chunk_pos: ChunkPosition) -> ChunkBuffer<bool> {
        let p = Vec2::new(12.3, 45.6);
        let value = simplex_noise_2d(p);
    }
}
