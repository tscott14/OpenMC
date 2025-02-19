// Given a IndexableChunkList, generate a vectors for the mesh verticies, normals, uvs, and indices

use cgmath::{Vector2, Vector3};

use crate::{
    block::block::Block,
    chunk::chunk::{IndexableChunkList, CHUNK_SIZE},
};

pub struct ChunkMesh {
    pub verticies: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl ChunkMesh {
    fn translate_vector(vertices: &[[f32; 3]; 4], index: &Vector3<f32>) -> [[f32; 3]; 4] {
        let vectorized = [
            Vector3::new(vertices[0][0], vertices[0][1], vertices[0][2]),
            Vector3::new(vertices[1][0], vertices[1][1], vertices[1][2]),
            Vector3::new(vertices[2][0], vertices[2][1], vertices[2][2]),
            Vector3::new(vertices[3][0], vertices[3][1], vertices[3][2]),
        ];

        let trans = [
            vectorized[0] + index,
            vectorized[1] + index,
            vectorized[2] + index,
            vectorized[3] + index,
        ];

        [
            [trans[0].x, trans[0].y, trans[0].z],
            [trans[1].x, trans[1].y, trans[1].z],
            [trans[2].x, trans[2].y, trans[2].z],
            [trans[3].x, trans[3].y, trans[3].z],
        ]
    }

    pub fn generate_mesh(list: &IndexableChunkList<Block>) -> ChunkMesh {
        // Define shared points
        const POINTS: [[f32; 3]; 8] = [
            [1.0, 1.0, 1.0], // 0: top right front
            [1.0, 1.0, 0.0], // 1: top right back
            [0.0, 1.0, 0.0], // 2: top left back
            [0.0, 1.0, 1.0], // 3: top left front
            [1.0, 0.0, 0.0], // 4: bottom right back
            [1.0, 0.0, 1.0], // 5: bottom right front
            [0.0, 0.0, 1.0], // 6: bottom left front
            [0.0, 0.0, 0.0], // 7: bottom left back
        ];

        // Define directional faces
        const VERTICIES_TOP: [[f32; 3]; 4] = [POINTS[0], POINTS[1], POINTS[2], POINTS[3]];
        const VERTICIES_BOTTOM: [[f32; 3]; 4] = [POINTS[4], POINTS[5], POINTS[6], POINTS[7]];
        const VERTICIES_FRONT: [[f32; 3]; 4] = [POINTS[6], POINTS[5], POINTS[0], POINTS[3]];
        const VERTICIES_BACK: [[f32; 3]; 4] = [POINTS[2], POINTS[1], POINTS[4], POINTS[7]];
        const VERTICIES_RIGHT: [[f32; 3]; 4] = [POINTS[1], POINTS[0], POINTS[5], POINTS[4]];
        const VERTICIES_LEFT: [[f32; 3]; 4] = [POINTS[3], POINTS[2], POINTS[7], POINTS[6]];

        // Define normals
        const NORMALS_TOP: [[f32; 3]; 4] = [[0.0, 1.0, 0.0]; 4];
        const NORMALS_BOTTOM: [[f32; 3]; 4] = [[0.0, -1.0, 0.0]; 4];
        const NORMALS_FRONT: [[f32; 3]; 4] = [[0.0, 0.0, 1.0]; 4];
        const NORMALS_BACK: [[f32; 3]; 4] = [[0.0, 0.0, -1.0]; 4];
        const NORMALS_RIGHT: [[f32; 3]; 4] = [[1.0, 0.0, 0.0]; 4];
        const NORMALS_LEFT: [[f32; 3]; 4] = [[-1.0, 0.0, 0.0]; 4];

        // Define uvs
        const UV_TOP: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        const UV_BOTTOM: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        const UV_FRONT: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        const UV_BACK: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        const UV_RIGHT: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        const UV_LEFT: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        

        // Define buffers
        let mut verticies = Vec::<[f32; 3]>::new();
        let mut normals = Vec::<[f32; 3]>::new();
        let mut uvs = Vec::<[f32; 2]>::new();
        let mut indices = Vec::<u32>::new();

        // loop through each block in the chunk
        for ((x, y, z), block) in list.iter() {
            if block.eq(&Block::Air) {
                continue;
            }

            let index_vectorized = Vector3::new(x as f32, y as f32, z as f32);

            let entries = [
                ((0, 1, 0), &VERTICIES_TOP, &NORMALS_TOP, &UV_TOP),
                ((0, -1, 0), &VERTICIES_BOTTOM, &NORMALS_BOTTOM, &UV_BOTTOM),
                ((0, 0, 1), &VERTICIES_FRONT, &NORMALS_FRONT, &UV_FRONT),
                ((0, 0, -1), &VERTICIES_BACK, &NORMALS_BACK, &UV_BACK),
                ((1, 0, 0), &VERTICIES_RIGHT, &NORMALS_RIGHT, &UV_RIGHT),
                ((-1, 0, 0), &VERTICIES_LEFT, &NORMALS_LEFT, &UV_LEFT),
            ];

            for ((dx, dy, dz), vert_param, normals_param, uvs_param) in entries {
                let (x, y, z) = (x as isize + dx, y as isize + dy, z as isize + dz);
                if list.try_get(x, y, z).unwrap_or(&Block::Air).eq(&Block::Air) {
                    let vector_trans = Self::translate_vector(vert_param, &index_vectorized);

                    let (u, v) = ((rand::random::<u8>() % 8) as f32 * 0.125, (rand::random::<u8>() % 8) as f32 * 0.125);
                    let trans_uvs = [
                        Vector2::new(uvs_param[0][0] / 8.0, uvs_param[0][1] / 8.0) + Vector2::new(u, v),
                        Vector2::new(uvs_param[1][0] / 8.0, uvs_param[1][1] / 8.0) + Vector2::new(u, v),
                        Vector2::new(uvs_param[2][0] / 8.0, uvs_param[2][1] / 8.0) + Vector2::new(u, v),
                        Vector2::new(uvs_param[3][0] / 8.0, uvs_param[3][1] / 8.0) + Vector2::new(u, v),
                    ];
                    let uvs_param_trans = [
                        [trans_uvs[0].x, trans_uvs[0].y],
                        [trans_uvs[1].x, trans_uvs[1].y],
                        [trans_uvs[2].x, trans_uvs[2].y],
                        [trans_uvs[3].x, trans_uvs[3].y],
                    ];

                    let vlen = verticies.len() as u32;
                    verticies.extend_from_slice(&vector_trans);
                    normals.extend_from_slice(normals_param);
                    uvs.extend_from_slice(&uvs_param_trans);
                    indices.extend_from_slice(&[
                        vlen,
                        vlen + 1,
                        vlen + 2,
                        vlen + 2,
                        vlen + 3,
                        vlen,
                    ]);
                }
            }
        }

        ChunkMesh {
            verticies,
            normals,
            uvs,
            indices,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::chunk::CHUNK_SIZE;

    use super::*;

    #[test]
    fn test_generate_mesh_empty() {
        let list = IndexableChunkList::<Block> {
            data: Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
        };
        let mesh = ChunkMesh::generate_mesh(&list);
        assert_eq!(mesh.verticies.len(), 0);
        assert_eq!(mesh.normals.len(), 0);
        assert_eq!(mesh.uvs.len(), 0);
        assert_eq!(mesh.indices.len(), 0);
    }

    #[test]
    fn test_generate_mesh_single() {
        let mut list = IndexableChunkList::<Block> {
            data: Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
        };

        list[(7, 7, 7)] = Block::Stone;

        let mesh = ChunkMesh::generate_mesh(&list);
        assert_eq!(mesh.verticies.len(), 24);
        assert_eq!(mesh.normals.len(), 24);
        assert_eq!(mesh.uvs.len(), 24);
        assert_eq!(mesh.indices.len(), 36);
    }

    #[test]
    fn test_generate_mesh_double_disconnected() {
        let mut list = IndexableChunkList::<Block> {
            data: Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]), 
        };

        list[(7, 7, 7)] = Block::Stone;
        list[(8, 8, 8)] = Block::Stone;

        let mesh = ChunkMesh::generate_mesh(&list);
        assert_eq!(mesh.verticies.len(), 48);
        assert_eq!(mesh.normals.len(), 48);
        assert_eq!(mesh.uvs.len(), 48);
        assert_eq!(mesh.indices.len(), 72);
    }

    #[test]
    fn test_generate_mesh_double_connected() {
        let mut list = IndexableChunkList::<Block> {
            data: Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
        };

        list[(7, 7, 7)] = Block::Stone;
        list[(7, 7, 8)] = Block::Stone;

        let mesh = ChunkMesh::generate_mesh(&list);
        assert_eq!(mesh.verticies.len(), 40);
        assert_eq!(mesh.normals.len(), 40);
        assert_eq!(mesh.uvs.len(), 40);
        assert_eq!(mesh.indices.len(), 60);
    }

    #[test]
    fn test_generate_mesh_double_encased() {
        let mut list = IndexableChunkList::<Block> {
            data: Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
        };

        list[(7, 7, 7)] = Block::Stone;
        list[(7, 8, 7)] = Block::Stone;
        list[(7, 6, 7)] = Block::Stone;
        list[(7, 7, 8)] = Block::Stone;
        list[(7, 7, 6)] = Block::Stone;
        list[(8, 7, 7)] = Block::Stone;
        list[(6, 7, 7)] = Block::Stone;

        let mesh = ChunkMesh::generate_mesh(&list);
        assert_eq!(mesh.verticies.len(), 6 * 20);
        assert_eq!(mesh.normals.len(), 6 * 20);
        assert_eq!(mesh.uvs.len(), 6 * 20);
        assert_eq!(mesh.indices.len(), 6 * 6 * 5);
    }
}
