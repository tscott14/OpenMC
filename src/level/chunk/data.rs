/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * June 3, 2024
 *
 * A class to represent chunk block data.
 */

use std::fmt::Debug;

use bevy::ecs::component::Component;

use crate::{block::block::Block, level::LevelPosition};

use super::{buffer::ChunkBuffer, config::CHUNK_SIZE};

#[derive(Component, Clone, Copy, Debug)]
pub struct ChunkBlockData(pub ChunkBuffer<Block>);

impl ChunkBlockData {
    /// Creates a new `ChunkBlockData` filled with the default `Block`.
    pub fn new() -> Self {
        Self::from_filled(&Block::Air)
    }

    /// Creates a new `ChunkBlockData` filled with the given `Block`.
    pub fn from_filled(block: &Block) -> Self {
        Self(ChunkBuffer::from_filled(block))
    }

    /// Sets the block at the given `LevelPosition` to the given `Block`.
    pub fn set(&mut self, position: LevelPosition, block: &Block) {
        self.0.set(position, block)
    }

    /// Returns the block at the given `LevelPosition`.
    pub fn get(&self, position: LevelPosition) -> &Block {
        self.0.get(position)
    }

    /// Returns the block at the given `LevelPosition`, if it is within the boundaries of the chunk.
    pub fn try_get(&self, position: LevelPosition) -> Option<&Block> {
        if LevelPosition::in_range(0, CHUNK_SIZE as i32, position.x)
            && LevelPosition::in_range(0, CHUNK_SIZE as i32, position.y)
            && LevelPosition::in_range(0, CHUNK_SIZE as i32, position.z)
        {
            return Some(self.get(position));
        }

        None
    }

    /// Fills the `ChunkBlockData` with the given `Block`.
    pub fn fill(&mut self, block: &Block) {
        self.0.fill(block)
    }
}
