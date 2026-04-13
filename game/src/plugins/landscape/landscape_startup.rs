// use bevy::prelude::*;

// pub fn startup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let chunk_mesh = Chunk::generate(ChunkMeshSettings {
//         verts_width: 20,
//         verts_length: 20,
//         vert_space_x: 1.0,
//         vert_space_z: 1.0,
//     });

//     let cube_mesh_handle = meshes.add(chunk_mesh.mesh);

//     commands.spawn((
//         Mesh3d(cube_mesh_handle),
//         MeshMaterial3d(materials.add(Color::WHITE)),
//         Transform::from_xyz(-475.0, 0.0, -675.0),
//     ));
// }
