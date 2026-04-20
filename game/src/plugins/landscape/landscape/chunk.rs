use std::sync::Arc;

use bevy::{
    asset::RenderAssetUsages,
    ecs::component::Component,
    log::error,
    math::{Vec2, Vec3},
    mesh::{Indices, Mesh, PrimitiveTopology},
    tasks::{AsyncComputeTaskPool, Task, block_on, futures_lite::future},
};

use crate::plugins::landscape::landscape::noise::{NoiseSettings, sample_noise};

#[derive(Component)]
pub struct Chunk {
    mesh: GeneratableChunkMesh,
    mesh_settings: ChunkMeshSettings,
    noise_settings: NoiseSettings,
    chunk_pos: Vec2,
    chunk_size: Vec2,
}

pub enum GeneratableChunkMesh {
    Generated,
    Generating(Task<Mesh>),
    Ungenerated,
}

impl PartialEq for GeneratableChunkMesh {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Generated, Self::Generated)
                | (Self::Generating(_), Self::Generating(_))
                | (Self::Ungenerated, Self::Ungenerated)
        )
    }
}

#[derive(Clone)]
pub struct ChunkMeshSettings {
    pub verts_width: usize,
    pub verts_length: usize,
    pub vert_space_x: f32,
    pub vert_space_z: f32,
}

impl Chunk {
    pub fn new(
        mesh_settings: ChunkMeshSettings,
        noise_settings: NoiseSettings,
        chunk_pos: Vec2,
    ) -> Self {
        Chunk {
            mesh: GeneratableChunkMesh::Ungenerated,
            chunk_pos,
            noise_settings,
            mesh_settings: mesh_settings.clone(),
            chunk_size: Vec2::new(
                (mesh_settings.verts_width - 1) as f32 * mesh_settings.vert_space_x,
                (mesh_settings.verts_length - 1) as f32 * mesh_settings.vert_space_z,
            ),
        }
    }

    pub fn _get_chunk_pos(&self) -> Vec2 {
        self.chunk_pos
    }

    pub fn get_chunk_size(&self) -> Vec2 {
        self.chunk_size
    }

    pub fn poll_chunk_generation_task(&mut self) -> Option<Mesh> {
        match &mut self.mesh {
            GeneratableChunkMesh::Generating(task) => {
                let poll = block_on(future::poll_once(task));

                match poll {
                    // Still generating
                    None => None,
                    // Finished generating
                    Some(mesh) => {
                        self.mesh = GeneratableChunkMesh::Generated;
                        Some(mesh)
                    }
                }
            }
            // Chunk is already generated
            _ => None,
        }
    }

    pub fn generate(mut self) -> Self {
        if self.mesh != GeneratableChunkMesh::Ungenerated {
            error!(
                "Tried to generate chunk at {}, but it is already generating/generated",
                self.chunk_pos
            );
            return self;
        }

        let mesh_settings = self.mesh_settings.clone();
        let noise_fn = Arc::clone(&self.noise_settings.noise_fn);
        let noise_x_freq_multiplier = self.noise_settings.noise_x_freq_multiplier;
        let chunk_pos = self.chunk_pos;
        let chunk_size = self.chunk_size;

        let task = AsyncComputeTaskPool::get().spawn(async move {
            // TODO: On web this work still blocks the main thread.
            // Investigate rust crates that utalize js web workers for true parrelism across multiple threads

            let mut verts = Vec::new();
            let mut normals = Vec::new();
            let mut uvs = vec![];

            let mut tris = Vec::new();

            for x in 0..mesh_settings.verts_width {
                for z in 0..mesh_settings.verts_length {
                    let vert = Vec3::new(
                        x as f32 * mesh_settings.vert_space_x,
                        sample_noise(
                            Vec2::new(
                                ((x as f32 * mesh_settings.vert_space_x)
                                    + (chunk_pos.x * chunk_size.x))
                                    * noise_x_freq_multiplier as f32,
                                (z as f32 * mesh_settings.vert_space_z)
                                    + (chunk_pos.y * chunk_size.y),
                            ),
                            noise_fn.as_ref(),
                        ),
                        z as f32 * mesh_settings.vert_space_z,
                    );
                    verts.push(vert);

                    let normal = Vec3::new(0.0, 1.0, 0.0);
                    normals.push(normal);

                    let uv = Vec2::new(
                        x as f32 * mesh_settings.vert_space_x,
                        z as f32 * mesh_settings.vert_space_z,
                    );

                    uvs.push(uv);

                    if x < mesh_settings.verts_width - 1 && z < mesh_settings.verts_length - 1 {
                        let i = x * mesh_settings.verts_length + z;
                        // first tri
                        tris.push(i as u32);
                        tris.push((i + 1) as u32);
                        tris.push((i + mesh_settings.verts_length + 1) as u32);
                        // second tri
                        tris.push((i + mesh_settings.verts_length + 1) as u32);
                        tris.push((i + mesh_settings.verts_length) as u32);
                        tris.push(i as u32);
                    }
                }
            }

            Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_indices(Indices::U32(tris))
        });

        self.mesh = GeneratableChunkMesh::Generating(task);
        self
    }

    // TODO: Function to sample y / height value from position?
}
