use bevy::prelude::*;

use crate::plugins::{
    landscape::landscape::{
        chunk::Chunk,
        controller::{LandscapeController, Tile},
    },
    player::Player,
};

pub fn start_generating_new_chunks(
    mut commands: Commands,
    mut landscape_controller: ResMut<LandscapeController>,
    player_transform: Single<&Transform, (With<Player>, Without<Camera3d>)>,
) {
    let player_position = player_transform.translation;

    let min_tile: usize = landscape_controller
        .world_position_to_tile(player_position)
        .into();
    let max_tile = min_tile
        + landscape_controller
            .settings
            .chunk_gen_settings
            .chunk_generation_distance;

    let tiles: Vec<Tile> = (min_tile..max_tile).collect();

    for tile in tiles {
        let chunk_mesh_settings = landscape_controller.settings.chunk_mesh_settings.clone();
        let chunk_noise_settings = landscape_controller.noise_settings.clone();

        landscape_controller
            .chunks
            .entry(tile.clone())
            .or_insert_with(|| {
                let new_chunk = Chunk::new(
                    chunk_mesh_settings.clone(),
                    chunk_noise_settings,
                    Vec2::new(0.0, tile as f32 * -1.0),
                )
                .generate();

                let chunk_depth = new_chunk.get_chunk_size();

                commands
                    .spawn((
                        new_chunk,
                        Transform::from_xyz(0.0, 0.0, tile as f32 * -chunk_depth.y),
                    ))
                    .id()
            });
    }
}

pub fn finish_generating_new_chunks(
    mut commands: Commands,
    landscape_controller: ResMut<LandscapeController>,
    player_transform: Single<&Transform, (With<Player>, Without<Camera3d>)>,
    mut chunks: Query<&mut Chunk>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_position = player_transform.translation;

    let min_tile: usize = landscape_controller
        .world_position_to_tile(player_position)
        .into();
    let max_tile = min_tile
        + landscape_controller
            .settings
            .chunk_gen_settings
            .chunk_generation_distance;

    let tiles: Vec<Tile> = (min_tile..max_tile).collect();

    for tile in tiles {
        let maybe_chunk_entity = landscape_controller.chunks.get(&tile);

        let chunk_entity = match maybe_chunk_entity {
            Some(chunk_entity) => chunk_entity.clone(),
            // Chunk doesn't exist yet
            None => continue,
        };

        let mut chunk_component = match chunks.get_mut(chunk_entity) {
            Ok(c) => c,
            // Entity is registered but not yet spawned (deferred command still pending),
            // or was despawned. Either way, skip this tile this frame.
            Err(_) => continue,
        };

        if let Some(mesh) = chunk_component.poll_chunk_generation_task() {
            let chunk_mesh_handle = meshes.add(mesh);

            commands.entity(chunk_entity).insert((
                Mesh3d(chunk_mesh_handle),
                MeshMaterial3d(materials.add(Color::WHITE)),
            ));
        }
    }
}
