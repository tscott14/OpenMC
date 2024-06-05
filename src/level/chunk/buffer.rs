/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * June 3, 2024
 *
 * An ADT for storing values of type `T` in a 3D grid.
 */

use std::fmt::Debug;

use crate::level::LevelPosition;

use super::config::CHUNK_SIZE;

/// A buffer for storing values of type `T` in a 3D grid.
#[derive(Copy, Clone, Default)]
pub struct ChunkBuffer<T: Copy + Default>(pub [[[T; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);

impl<T: Copy + Default> ChunkBuffer<T> {
    /// Creates a new `Buffer` filled with the given value.
    pub fn from_item(value: &T) -> Self {
        let mut buffer = Self::default();
        buffer.fill(value);
        return buffer;
    }

    /// Creates a new `Buffer` from an array.
    pub fn from_array(data: [[[T; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) -> Self {
        Self(data)
    }

    /// Creates a new `Buffer` from a function.
    pub fn from_fn(f: impl Fn(LevelPosition) -> T) -> Self {
        ChunkBuffer::<()>::default()
            .iter()
            .fold(Self::default(), |mut buffer, (pos, _)| {
                buffer.set(pos, &f(pos));
                buffer
            })
    }

    /// Sets the value at the given `LevelPosition` to the given value.
    pub fn set(&mut self, position: LevelPosition, value: &T) {
        let (x, y, z) = position.get_xyz();
        self.0[z as usize][x as usize][y as usize] = *value;
    }

    /// Returns the value at the given `LevelPosition`.
    pub fn get(&self, position: LevelPosition) -> &T {
        let (x, y, z) = position.get_xyz();
        &self.0[z as usize][x as usize][y as usize]
    }

    /// Fills the `Buffer` with the given value.
    pub fn fill(&mut self, value: &T) {
        ChunkBuffer::<()>::default()
            .iter()
            .for_each(|(pos, _)| self.set(pos, value));
    }

    /// Clears the `Buffer` by resetting all values to their default.
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

pub struct ChunkBufferIter<'a, T: Copy + Default> {
    buffer: &'a ChunkBuffer<T>,
    index: usize,
}

impl<'a, T: Copy + Default> Iterator for ChunkBufferIter<'a, T> {
    type Item = (LevelPosition, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        const CHUNK_VOLUME_SIZE: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
        const CHUNK_PLANE_SIZE: usize = CHUNK_SIZE * CHUNK_SIZE;

        if self.index >= CHUNK_VOLUME_SIZE {
            return None;
        }

        let (y_pos, xz_space) = (self.index / CHUNK_PLANE_SIZE, self.index % CHUNK_PLANE_SIZE);
        let (x_pos, z_pos) = (xz_space / CHUNK_SIZE, xz_space % CHUNK_SIZE);

        self.index += 1;

        let position = LevelPosition::new(x_pos as i32, y_pos as i32, z_pos as i32);
        Some((position, &self.buffer.get(position)))
    }
}

impl<T: Copy + Default> ChunkBuffer<T> {
    /// Creates an iterator over the contents of the `Buffer`.
    pub fn iter(&self) -> ChunkBufferIter<'_, T> {
        ChunkBufferIter {
            buffer: self,
            index: 0,
        }
    }
}

impl<T: Copy + Debug + Default> Debug for ChunkBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(ChunkBuffer::<()>::default().iter().for_each(|(position, _)| {
            let (x, y, z) = position.get_xyz();
            let value = self.get(position);

            match (x, z) {
                (0, 0) => {
                    writeln!(f, "Chunk Slice at y={}", y).unwrap();
                    write!(f, "{:?} ", value).unwrap();
                }
                (_, 3) => writeln!(f, "{:?} ", value).unwrap(),
                (_, _) => write!(f, "{:?} ", value).unwrap(),
            };
        }))
    }
}


// impl<T: Copy + Default, U: Copy + Default> ChunkBuffer<T> {
//     /// Applies a function to each element in the `Buffer` and returns a new `Buffer`
//     /// with the results.
//     pub fn map<F: Fn(LevelPosition, T) -> U>(&self, func: F) -> ChunkBuffer<U> {
//         let mut new_buffer = ChunkBuffer::default();

//         for (position, value) in self.iter() {
//             new_buffer.set(position, &func(position, *value));
//         }

//         new_buffer
//     }
// }
