use super::global::GlobalPosition;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct ChunkUnit(i32);

impl From<i32> for ChunkUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<ChunkUnit> for i32 {
    fn from(value: ChunkUnit) -> Self {
        value.0
    }
}

#[derive(Component, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ChunkPosition(ChunkUnit, ChunkUnit, ChunkUnit);

/// Basic functions for ChunkPosition.
impl ChunkPosition {
    pub fn offset(&self, other: Self) -> Self {
        Self::from((
            self.0.into() + other.0.into(),
            self.1.into() + other.1.into(),
            self.2.into() + other.2.into(),
        ))
    }
}

/// Convert a (ChunkUnit, ChunkUnit, ChunkUnit) into a ChunkPosition.
impl From<(ChunkUnit, ChunkUnit, ChunkUnit)> for ChunkPosition {
    fn from(position: (ChunkUnit, ChunkUnit, ChunkUnit)) -> Self {
        Self(position.0, position.1, position.2)
    }
}

/// Convert a ChunkPosition into a non-adjusted (ChunkUnit, ChunkUnit, ChunkUnit).
impl From<ChunkPosition> for (ChunkUnit, ChunkUnit, ChunkUnit) {
    fn from(position: ChunkPosition) -> Self {
        (position.0, position.1, position.2)
    }
}

/// Convert a GlobalPosition into a truncated ChunkPosition.
impl From<GlobalPosition> for ChunkPosition {
    fn from(position: GlobalPosition) -> Self {
        const CHUNK_SIZE: usize = crate::level::config::Configs::ChunkSize as usize;
        let (gx, gy, gz) = position.into();
        Self::from((
            ChunkUnit::from(gx.div_euclid(CHUNK_SIZE as i32)),
            ChunkUnit::from(gy.div_euclid(CHUNK_SIZE as i32)),
            ChunkUnit::from(gz.div_euclid(CHUNK_SIZE as i32)),
        ))
    }
}

/// Convert from a global position into a interchunk chunk position and a intrachunk global position.
impl From<GlobalPosition> for (ChunkPosition, GlobalPosition) {
    fn from(position: GlobalPosition) -> Self {
        const CHUNK_SIZE: usize = crate::level::config::Configs::ChunkSize as usize;
        let (gx, gy, gz) = position.into();
        (
            ChunkPosition::from((
                gx.div_euclid(CHUNK_SIZE as i32),
                gy.div_euclid(CHUNK_SIZE as i32),
                gz.div_euclid(CHUNK_SIZE as i32),
            )),
            GlobalPosition::from((
                gx.rem_euclid(CHUNK_SIZE as i32),
                gy.rem_euclid(CHUNK_SIZE as i32),
                gz.rem_euclid(CHUNK_SIZE as i32),
            )),
        )
    }
}

/// Display a ChunkPosition.
impl std::fmt::Display for ChunkPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (cx, cy, cz) = <(ChunkUnit, ChunkUnit, ChunkUnit)>::from(*self);
        write!(f, "[{}, {}, {}]", cx.into(), cy.into(), cz.into())
    }
}

/// Debug a ChunkPosition.
impl std::fmt::Debug for ChunkPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (cx, cy, cz) = <(ChunkUnit, ChunkUnit, ChunkUnit)>::from(*self);
        write!(f, "ChunkPosition: [x: {}, y: {}, z: {}]", cx.into(), cy.into(), cz.into())
    }
}
