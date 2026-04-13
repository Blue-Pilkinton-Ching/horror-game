use bevy::{
    asset::RenderAssetUsages,
    ecs::component::Component,
    math::{Vec2, Vec3, ops::sin},
    mesh::{Indices, Mesh, PrimitiveTopology},
    tasks::{AsyncComputeTaskPool, Task},
};

#[derive(Component)]
pub struct Chunk {
    pub mesh: GeneratableChunkMesh,
    pub mesh_settings: ChunkMeshSettings,
}

pub enum GeneratableChunkMesh {
    Generated,
    Generating(Task<Mesh>),
    Ungenerated,
}

#[derive(Clone)]
pub struct ChunkMeshSettings {
    pub verts_width: usize,
    pub verts_length: usize,
    pub vert_space_x: f32,
    pub vert_space_z: f32,
}

impl Chunk {
    pub fn new(mesh_settings: ChunkMeshSettings) -> Self {
        Chunk {
            mesh: GeneratableChunkMesh::Ungenerated,
            mesh_settings,
        }
    }

    pub fn generate(mut self) -> Self {
        let task = AsyncComputeTaskPool::get().spawn(async move {
            // TODO: On web this work still blocks the main thread.
            // Investigate rust crates that utalize js web workers for true parrelism across multiple threads

            let mut verts = Vec::new();
            let mut normals = Vec::new();
            let mut uvs = vec![];

            let mut tris = Vec::new();

            for x in 0..self.mesh_settings.verts_width {
                for z in 0..self.mesh_settings.verts_length {
                    let vert = Vec3::new(
                        x as f32 * self.mesh_settings.vert_space_x,
                        sin(x as f32 * 1.0 + z as f32 * 1.0) as f32 * 1.0,
                        z as f32 * self.mesh_settings.vert_space_z,
                    );
                    verts.push(vert);

                    let normal = Vec3::new(0.0, 1.0, 0.0);
                    normals.push(normal);

                    let uv = Vec2::new(
                        x as f32 * self.mesh_settings.vert_space_x,
                        z as f32 * self.mesh_settings.vert_space_z,
                    );

                    uvs.push(uv);

                    if x < self.mesh_settings.verts_width - 1
                        && z < self.mesh_settings.verts_length - 1
                    {
                        let i = x * self.mesh_settings.verts_length + z;
                        // first tri
                        tris.push(i as u32);
                        tris.push((i + 1) as u32);
                        tris.push((i + self.mesh_settings.verts_length + 1) as u32);
                        // second tri
                        tris.push((i + self.mesh_settings.verts_length + 1) as u32);
                        tris.push((i + self.mesh_settings.verts_length) as u32);
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
