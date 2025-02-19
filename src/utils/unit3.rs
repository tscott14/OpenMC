use std::ops::Mul;


use cgmath::{num_traits::Euclid, Vector3};

#[derive(Clone, Debug, PartialEq)]
pub struct BlockUnits<T>(Vector3<T>);
#[derive(Clone, Debug, PartialEq)]
pub struct ChunkUnits<T>(Vector3<T>);

impl<T> From<BlockUnits<T>> for ChunkUnits<T> {
    fn from(value: BlockUnits<T>) -> Self {
        Self(value.0)
    }
}

impl<T> From<ChunkUnits<T>> for BlockUnits<T> {
    fn from(value: ChunkUnits<T>) -> Self {
        Self(value.0)
    }
}

#[derive(Debug, PartialEq)]
pub enum Vector3i<T> {
    Block(BlockUnits<T>),
    Chunk(ChunkUnits<T>),
}

impl<T: cgmath::num_traits::Euclid> Vector3i<T>
where
    T: Mul<T, Output = T> + Copy,
{
    pub fn get_xyz(&self) -> (T, T, T) {
        match self {
            Self::Block(unit) => (unit.0.x, unit.0.y, unit.0.z),
            Self::Chunk(unit) => (unit.0.x, unit.0.y, unit.0.z),
        }
    }

    
    pub fn block(&self) -> Self {
        match self {
            Self::Block(unit) => Self::Block(BlockUnits::<T>(unit.0.clone())),
            Self::Chunk(unit) => Self::Block(BlockUnits::<T>::from(unit.clone())),
        }
    }

    pub fn chunk(&self) -> Self {
        match self {
            Self::Block(unit) => Self::Chunk(ChunkUnits::<T>::from(unit.clone())),
            Self::Chunk(unit) => Self::Chunk(unit.clone()),
        }
    }

    pub fn block_units(&self) -> BlockUnits<T>
    where
        T: Euclid + TryFrom<usize>,
    {
        use crate::chunk::chunk::CHUNK_SIZE;
        match self {
            Self::Block(unit) => unit.clone(),
            Self::Chunk(unit) => BlockUnits(
                unit.0
                    .map(|x| x.mul(T::try_from(CHUNK_SIZE).ok().expect("Invalid Chunk Size"))),
            ),
        }
    }

    pub fn chunk_units(&self) -> ChunkUnits<T>
    where
        T: Euclid + TryFrom<usize>,
    {
        use crate::chunk::chunk::CHUNK_SIZE;
        match self {
            Self::Block(unit) => {
                ChunkUnits(unit.0.map(|x| {
                    x.div_euclid(&T::try_from(CHUNK_SIZE).ok().expect("Invalid Chunk Size"))
                }))
            }
            Self::Chunk(unit) => unit.clone(),
        }
    }
}

pub type Vector3i32 = Vector3i<i32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_xyz() {
        let block = Vector3i::<i32>::Block(BlockUnits::<i32>(Vector3::<i32>::new(1, 2, 3)));
        let chunk = Vector3i::<i32>::Chunk(ChunkUnits::<i32>(Vector3::<i32>::new(4, 5, 6)));

        assert_eq!(block.get_xyz(), (1, 2, 3));
        assert_eq!(chunk.get_xyz(), (4, 5, 6));
    }

    #[test]
    fn test_block() {
        let block = Vector3i::<i32>::Block(BlockUnits::<i32>(Vector3::<i32>::new(1, 2, 3)));
        let chunk = Vector3i::<i32>::Chunk(ChunkUnits::<i32>(Vector3::<i32>::new(4, 5, 6)));

        assert_eq!(block.block(), block);
        assert_eq!(
            chunk.block(),
            Vector3i::<i32>::Block(BlockUnits::<i32>(Vector3::<i32>::new(4, 5, 6)))
        );
    }

    #[test]
    fn test_chunk() {
        let block = Vector3i::<i32>::Block(BlockUnits::<i32>(Vector3::<i32>::new(1, 2, 3)));
        let chunk = Vector3i::<i32>::Chunk(ChunkUnits::<i32>(Vector3::<i32>::new(4, 5, 6)));

        assert_eq!(
            block.chunk(),
            Vector3i::<i32>::Chunk(ChunkUnits::<i32>(Vector3::<i32>::new(1, 2, 3)))
        );
        assert_eq!(chunk.chunk(), chunk);
    }

    #[test]
    fn test_block_units() {
        let block = Vector3i::<i32>::Block(BlockUnits::<i32>(Vector3::<i32>::new(15, 16, 17)));
        let chunk = Vector3i::<i32>::Chunk(ChunkUnits::<i32>(Vector3::<i32>::new(4, 5, 6)));

        assert_eq!(
            block.block_units(),
            BlockUnits::<i32>(Vector3::<i32>::new(15, 16, 17))
        );
        assert_eq!(
            chunk.block_units(),
            BlockUnits::<i32>(Vector3::<i32>::new(64, 80, 96))
        );
    }

    #[test]
    fn test_chunk_units() {
        let block = Vector3i::<i32>::Block(BlockUnits::<i32>(Vector3::<i32>::new(15, 16, 17)));
        let chunk = Vector3i::<i32>::Chunk(ChunkUnits::<i32>(Vector3::<i32>::new(4, 5, 6)));

        assert_eq!(
            block.chunk_units(),
            ChunkUnits::<i32>(Vector3::<i32>::new(0, 1, 1))
        );
        assert_eq!(
            chunk.chunk_units(),
            ChunkUnits::<i32>(Vector3::<i32>::new(4, 5, 6))
        );
    }
}
