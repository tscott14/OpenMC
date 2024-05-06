/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * May 9, 2024
 *
 * A Utility class to generate optomized block data for rendering.
 */

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BlockFaceCodes {
    BlockFacePositiveX,
    BlockFaceNegativeX,
    BlockFacePositiveY,
    BlockFaceNegativeY,
    BlockFacePositiveZ,
    BlockFaceNegativeZ,
}

impl BlockFaceCodes {
    /// Returns the `BlockFaceCodes` as a `u8`.
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    /// Returns the `BlockFaceCodes` corresponding to the given `u8` code.
    ///
    /// # Panics
    ///
    /// If the given `code` is invalid.
    pub fn from_u8(code: u8) -> Self {
        match code {
            0x0 => Self::BlockFacePositiveX,
            0x1 => Self::BlockFaceNegativeX,
            0x2 => Self::BlockFacePositiveY,
            0x3 => Self::BlockFaceNegativeY,
            0x4 => Self::BlockFacePositiveZ,
            0x5 => Self::BlockFaceNegativeZ,
            _ => panic!("Invalid block face code"),
        }
    }

    /// Returns a `u8` mask corresponding to the `BlockFaceCodes`.
    pub fn mask_u8(&self) -> u8 {
        1 << self.as_u8()
    }

    /// Returns an array of all the `BlockFaceCodes` variants.
    pub fn variants() -> [Self; 6] {
        (0..6)
            .map(|i| Self::from_u8(i))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    /// Returns an array of vertex data corresponding to the `BlockFaceCodes`.
    pub fn vertex_data(&self) -> [[f32; 3]; 4] {
        match self {
            Self::BlockFacePositiveX => [
                [1.0, 1.0, 0.0],
                [1.0, 1.0, 1.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 1.0],
            ],
            Self::BlockFaceNegativeX => [
                [0.0, 1.0, 1.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0],
            ],
            Self::BlockFacePositiveY => [
                [0.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
            ],
            Self::BlockFaceNegativeY => [
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 1.0],
            ],
            Self::BlockFacePositiveZ => [
                [1.0, 1.0, 1.0],
                [0.0, 1.0, 1.0],
                [1.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
            Self::BlockFaceNegativeZ => [
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
            ],
        }
    }

    /// Returns an array of texture data corresponding to the `BlockFaceCodes`.
    pub fn texture_data(&self) -> [[f32; 2]; 4] {
        match self {
            Self::BlockFacePositiveX => [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
            Self::BlockFaceNegativeX => [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
            Self::BlockFacePositiveY => [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
            Self::BlockFaceNegativeY => [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
            Self::BlockFacePositiveZ => [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
            Self::BlockFaceNegativeZ => [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
        }
    }

    /// Returns an array of normal data corresponding to the `BlockFaceCodes`.
    pub fn normal_data(&self) -> [f32; 3] {
        match self {
            Self::BlockFacePositiveX => [1.0, 0.0, 0.0],
            Self::BlockFaceNegativeX => [-1.0, 0.0, 0.0],
            Self::BlockFacePositiveY => [0.0, 1.0, 0.0],
            Self::BlockFaceNegativeY => [0.0, -1.0, 0.0],
            Self::BlockFacePositiveZ => [0.0, 0.0, 1.0],
            Self::BlockFaceNegativeZ => [0.0, 0.0, -1.0],
        }
    }

    pub fn direction(&self) -> (i32, i32, i32) {
        match self {
            Self::BlockFacePositiveX => (1, 0, 0),
            Self::BlockFaceNegativeX => (-1, 0, 0),
            Self::BlockFacePositiveY => (0, 1, 0),
            Self::BlockFaceNegativeY => (0, -1, 0),
            Self::BlockFacePositiveZ => (0, 0, 1),
            Self::BlockFaceNegativeZ => (0, 0, -1),
        }
    }
}
