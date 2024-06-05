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

#[derive(Component, Clone, Copy, Default, Debug)]
pub struct ChunkBlockData(pub ChunkBuffer<Block>);

impl ChunkBlockData {
    /// Creates a new `ChunkBlockData` filled with the given `Block`.
    pub fn from_item(block: &Block) -> Self {
        Self(ChunkBuffer::from_item(block))
    }

    /// Creates a new `ChunkBlockData` from an array.
    pub fn from_array(data: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) -> Self {
        Self(ChunkBuffer::from_array(data))
    }

    /// Creates a new `ChunkBlockData` from a function.
    pub fn from_fn(chunk_position: LevelPosition, f: impl Fn(LevelPosition, LevelPosition) -> Block) -> Self {
        Self(ChunkBuffer::from_fn(|pos| {
            let (x, y, z) = pos.get_xyz();
            f(chunk_position.offset(x, y, z), pos)
        }))
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

