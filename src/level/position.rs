/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * June 3, 2024
 *
 * An easy way to represent a position in the world.
 */

use std::fmt::{Debug, Display};

use bevy::{ecs::component::Component, math::Vec3};

use super::chunk::config::CHUNK_SIZE;

/// A struct representing a 3d position in the world.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct LevelPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl LevelPosition {
    /// Creates a new `LevelPosition` from the given coordinates.
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Returns the x, y, and z coordinates as a tuple.
    pub fn get_xyz(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }

    /// Returns a new `LevelPosition` that is offset by the given coordinates.
    pub fn offset(&self, x: i32, y: i32, z: i32) -> Self {
        Self::new(self.x + x, self.y + y, self.z + z)
    }

    /// Returns the chunk x, y, and z coordinates of the position.
    pub fn chunk_xyz(&self) -> (i32, i32, i32) {
        (
            self.x / CHUNK_SIZE as i32,
            self.y / CHUNK_SIZE as i32,
            self.z / CHUNK_SIZE as i32,
        )
    }

    /// Returns the chunk local x, y, and z coordinates of the position.
    pub fn chunk_dxdydz(&self) -> (i32, i32, i32) {
        (
            self.x % CHUNK_SIZE as i32,
            self.y % CHUNK_SIZE as i32,
            self.z % CHUNK_SIZE as i32,
        )
    }

    /// Returns a new `LevelPosition` that is the start of the chunk at the given coordinates.
    pub fn from_chunk_xyz(x: i32, y: i32, z: i32) -> Self {
        Self::new(
            x * CHUNK_SIZE as i32,
            y * CHUNK_SIZE as i32,
            z * CHUNK_SIZE as i32,
        )
    }

    /// Returns whether the given value is within the range [min, max).
    pub fn in_range(min: i32, max: i32, value: i32) -> bool {
        value >= min && value < max
    }
}

impl Display for LevelPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LevelPosition[x:{}, y:{}, z:{}]", self.x, self.y, self.z)
    }
}

impl Debug for LevelPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LevelPosition[[x:{}, y:{}, z:{}]]",
            self.x, self.y, self.z
        )
    }
}

