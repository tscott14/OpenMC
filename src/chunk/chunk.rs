use std::ops::{Index, IndexMut};

use crate::block::block::Block;

pub const CHUNK_SIZE: usize = 16;
const CHUNK_SIZE2: usize = CHUNK_SIZE * CHUNK_SIZE;

trait ChunkIndexable: Index<(usize, usize, usize)> {}

pub struct IndexableChunkList<T> {
    pub(crate) data: Box<[[[T; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
}

impl<T> IndexableChunkList<T> {
    pub fn iter(&self) -> IndexableChunkListIterator<T> {
        IndexableChunkListIterator {
            inner: self,
            index: 0,
        }
    }

    pub fn try_get(&self, x: isize, y: isize, z: isize) -> Option<&T> {
        if x < 0
            || x >= CHUNK_SIZE as isize
            || y < 0
            || y >= CHUNK_SIZE as isize
            || z < 0
            || z >= CHUNK_SIZE as isize
        {
            return None;
        }

        let (x, y, z) = (x as usize, y as usize, z as usize);
        Some(self.data.get(y)?.get(z)?.get(x)?)
    }
}

impl ChunkIndexable for IndexableChunkList<Block> {}

impl<T> Index<(usize, usize, usize)> for IndexableChunkList<T> {
    type Output = T;
    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        let (x, y, z) = index;
        self.data
            .get(y)
            .expect(format!("Y index out of bounds: [y={}]", y).as_str())
            .get(z)
            .expect(format!("Z index out of bounds: [z={}]", z).as_str())
            .get(x)
            .expect(format!("X index out of bounds: [x={}]", x).as_str())
    }
}

impl<T> IndexMut<(usize, usize, usize)> for IndexableChunkList<T> {
    // type Output = T;
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        let (x, y, z) = index;
        self.data
            .get_mut(y)
            .expect(format!("Y index out of bounds: [y={}]", y).as_str())
            .get_mut(z)
            .expect(format!("Z index out of bounds: [z={}]", z).as_str())
            .get_mut(x)
            .expect(format!("X index out of bounds: [x={}]", x).as_str())
    }
}

pub struct IndexableChunkListIterator<'a, T> {
    inner: &'a IndexableChunkList<T>,
    index: usize,
}

impl<'a, T> Iterator for IndexableChunkListIterator<'a, T> {
    type Item = ((usize, usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let (y, r) = (self.index / CHUNK_SIZE2, self.index % CHUNK_SIZE2);
        let (z, x) = (r / CHUNK_SIZE, r % CHUNK_SIZE);

        if x >= CHUNK_SIZE || y >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return None;
        }

        self.index += 1;
        Some(((x, y, z), &self.inner[(x, y, z)]))
    }
}

pub struct ChunkData {
    data: IndexableChunkList<Block>,
}

impl ChunkData {
    pub fn new() -> ChunkData {
        ChunkData {
            data: IndexableChunkList {
                data: Box::new([[[Block::Stone; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
            },
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Block {
        self.data[(x, y, z)]
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) -> &mut Self {
        self.data[(x, y, z)] = block;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexable_chunk_list_new() {
        let chunk_list = IndexableChunkList::<Block> {
            data: Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
        };
        assert_eq!(chunk_list.data.len(), CHUNK_SIZE);
        assert_eq!(chunk_list.data[0].len(), CHUNK_SIZE);
        assert_eq!(chunk_list.data[0][0].len(), CHUNK_SIZE);
        assert_eq!(chunk_list[(0, 0, 0)], Block::Air);
    }

    #[test]
    fn test_indexable_chunk_list_get() {
        let mut chunk_list = IndexableChunkList::<Block> {
            data: Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
        };
        chunk_list[(0, 0, 0)] = Block::Stone;
        assert_eq!(chunk_list[(0, 0, 0)], Block::Stone);
        assert_eq!(chunk_list[(0, 0, 1)], Block::Air);
    }

    #[test]
    fn test_indexable_chunk_list_set() {
        let mut chunk_list = IndexableChunkList::<Block> {
            data: Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
        };
        chunk_list[(0, 0, 0)] = Block::Stone;
        assert_eq!(chunk_list[(0, 0, 0)], Block::Stone);
    }

    #[test]
    fn test_indexable_chunk_list_cycle() {
        let chunk_list = IndexableChunkList::<Block> {
            data: Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
        };
        let mut iter = chunk_list.iter();

        let mut count = 0;
        for n in [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] {
            count += n + 1;
            // Skip n elements
            (0..n).for_each(|_| {
                iter.next();
            });
            if let Some(((x, y, z), _)) = iter.next() {
                let index = y * (CHUNK_SIZE * CHUNK_SIZE) + z * CHUNK_SIZE + x;
                assert_eq!(index, count - 1);
            }
        }
    }

    #[test]
    fn test_indexable_chunk_list_iter() {
        let mut chunk_list = IndexableChunkList::<Block> {
            data: Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
        };
        chunk_list[(0, 0, 0)] = Block::Stone;
        let mut iter = chunk_list.iter();
        assert_eq!(iter.next(), Some(((0, 0, 0), &Block::Stone)));
        assert_eq!(iter.next(), Some(((1, 0, 0), &Block::Air)));
    }
}
