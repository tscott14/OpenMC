use crate::utils::position::global::GlobalPosition;

use super::config::Configs;

/// An index into a `Buffer`.
///
/// This is used to access a single block within a `Buffer`.
pub struct BufferIndex(usize);

impl BufferIndex {
    /// The maximum value for a `BufferIndex`.
    ///
    /// This is the number of blocks in a `Buffer`.
    pub const MAX: usize = Configs::ChunkSize * Configs::ChunkSize * Configs::ChunkSize;

    /// The minimum value for a `BufferIndex`.
    ///
    /// This is the index of the first block in a `Buffer`.
    pub const MIN: usize = 0;

    /// Returns the value of the `BufferIndex`.
    pub fn value(&self) -> usize {
        self.0
    }
}

impl From<usize> for BufferIndex {
    /// Converts a `usize` into a `BufferIndex`.
    ///
    /// # Panics
    ///
    /// This function will panic if the `usize` is greater than `BufferIndex::MAX` or less than `BufferIndex::MIN`.
    fn from(index: usize) -> Self {
        debug_assert!(index < BufferIndex::MAX);
        debug_assert!(index >= BufferIndex::MIN);
        Self(index)
    }
}

impl From<BufferIndex> for usize {
    /// Converts a `BufferIndex` into a `usize`.
    fn from(index: BufferIndex) -> Self {
        index.0
    }
}

pub struct ChunkBuffer<T: Copy + Default>(
    [T; Configs::ChunkSize as usize * Configs::ChunkSize as usize * Configs::ChunkSize as usize],
);

impl<T: Copy + Default> ChunkBuffer<T> {
    const SIZE: usize = Configs::ChunkSize * Configs::ChunkSize * Configs::ChunkSize;
    pub fn new() -> Self {
        Self(
            [T::default(); Self::SIZE]
        )
    }

    pub fn get(&self, position: GlobalPosition) -> &T {
        let (_interchunk, intrachunk) = position.into();
        let index = intrachunk.into();
        self.0.get(index.value()).expect(format!("An invalid index was given: {index}").as_str())
    }

    pub fn set(&mut self, position: GlobalPosition, value: &T) {
        let (_interchunk, intrachunk) = position.into();
        let index = intrachunk.into();
        self.0[index.value()] = *value;
    }
}
