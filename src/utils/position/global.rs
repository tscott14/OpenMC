use std::ops::Range;

use crate::level::buffer::BufferIndex;

use super::chunk::ChunkPosition;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct GlobalUnit(i32);

impl From<i32> for GlobalUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<GlobalUnit> for i32 {
    fn from(value: GlobalUnit) -> Self {
        value.0
    }
}

// Represents a 1-1 relation between units.
#[derive(Component, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct GlobalPosition(GlobalUnit, GlobalUnit, GlobalUnit);

/// Basic functions for GlobalPosition.
impl GlobalPosition {
    pub fn offset(&self, other: Self) -> Self {
        Self::from((
            self.0.into() + other.0.into(),
            self.1.into() + other.1.into(),
            self.2.into() + other.2.into(),
        ))
    }
}

/// Convert a (GlobalUnit, GlobalUnit, GlobalUnit) into a GlobalPosition.
impl From<(GlobalUnit, GlobalUnit, GlobalUnit)> for GlobalPosition {
    fn from(position: (GlobalUnit, GlobalUnit, GlobalUnit)) -> Self {
        Self(position.0, position.1, position.2)
    }
}

/// Convert a GlobalPosition into a non-adjusted (GlobalUnit, GlobalUnit, GlobalUnit).
impl From<GlobalPosition> for (GlobalUnit, GlobalUnit, GlobalUnit) {
    fn from(position: GlobalPosition) -> Self {
        (position.0, position.1, position.2)
    }
}

/// Convert a ChunkPosition into a GlobalPosition.
impl From<BufferIndex> for GlobalPosition {
    fn from(index: BufferIndex) -> Self {
        const CHUNK_SIZE: usize = crate::level::config::Configs::ChunkSize as usize;

        debug_assert!(index < BufferIndex::MAX);

        let (rzy, x) = (index / CHUNK_SIZE, index % CHUNK_SIZE);
        let (y, z) = (rzy / CHUNK_SIZE, rzy % CHUNK_SIZE);
        Self(
            GlobalUnit::from(x),
            GlobalUnit::from(y),
            GlobalUnit::from(z),
        )
    }
}

/// Convert a GlobalPosition into a ChunkPosition.
impl From<GlobalPosition> for crate::level::buffer::BufferIndex {
    fn from(position: GlobalPosition) -> Self {
        const CHUNK_SIZE: usize = crate::level::config::Configs::ChunkSize as usize;
        let (x, y, z) = position.into();
        let index = BufferIndex::from(
            x.into() + (z.into() * CHUNK_SIZE) + (y.into() * CHUNK_SIZE * CHUNK_SIZE),
        );
        debug_assert!(index < BufferIndex::MAX);
        index
    }
}

/// Convert a ChunkPosition into a GlobalPosition.
impl From<ChunkPosition> for GlobalPosition {
    fn from(position: ChunkPosition) -> Self {
        const CHUNK_SIZE: usize = crate::level::config::Configs::ChunkSize as usize;
        let (cx, cy, cz) = position.into();
        Self::from((
            GlobalUnit::from(cx.into() * CHUNK_SIZE as i32),
            GlobalUnit::from(cy.into() * CHUNK_SIZE as i32),
            GlobalUnit::from(cz.into() * CHUNK_SIZE as i32),
        ))
    }
}

/// Convert from a chunk position and a relative global position into a combined global position.
impl From<(ChunkPosition, GlobalPosition)> for GlobalPosition {
    fn from((chunk, global): (ChunkPosition, GlobalPosition)) -> Self {
        const CHUNK_SIZE: usize = crate::level::config::Configs::ChunkSize as usize;
        let (cx, cy, cz) = chunk.into();
        let (rx, ry, rz) = global.into();

        // rx, ry, rz should always be in the range [0, CHUNK_SIZE)!
        debug_assert!((0..CHUNK_SIZE as i32).contains(&rx.into()));
        debug_assert!((0..CHUNK_SIZE as i32).contains(&ry.into()));
        debug_assert!((0..CHUNK_SIZE as i32).contains(&rz.into()));
        Self::from((
            GlobalUnit::from(cx.into() * CHUNK_SIZE as i32 + rx),
            GlobalUnit::from(cy.into() * CHUNK_SIZE as i32 + ry),
            GlobalUnit::from(cz.into() * CHUNK_SIZE as i32 + rz),
        ))
    }
}

/// Display a GlobalPosition.
impl std::fmt::Display for GlobalPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (gx, gy, gz) = <(GlobalUnit, GlobalUnit, GlobalUnit)>::from(*self);
        write!(f, "[{}, {}, {}]", gx.into(), gy.into(), gz.into())
    }
}

/// Debug a GlobalPosition.
impl std::fmt::Debug for GlobalPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (gx, gy, gz) = <(GlobalUnit, GlobalUnit, GlobalUnit)>::from(*self);
        write!(
            f,
            "GlobalPosition: [x: {}, y: {}, z: {}]",
            gx.into(),
            gy.into(),
            gz.into()
        )
    }
}
