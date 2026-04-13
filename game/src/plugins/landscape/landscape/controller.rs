use std::collections::HashMap;

use bevy::{
    ecs::{entity::Entity, resource::Resource},
    math::*,
};

use crate::plugins::landscape::landscape::chunk::ChunkMeshSettings;

#[derive(Resource)]
pub struct LandscapeController {
    pub settings: LandscapeControllerSettings,
    // Chunks are keyed by their tile position
    pub chunks: HashMap<Tile, ChunkEntity>,
}

pub type ChunkEntity = Entity;

// The tile is the position of the chunk.
// Chunks can only generate towards neg z, so this is the distance from the origin toward neg z in chunks.
pub type Tile = usize;

pub struct LandscapeControllerSettings {
    pub chunk_mesh_settings: ChunkMeshSettings,
    pub chunk_gen_settings: ChunkGenSettings,
}

pub struct ChunkGenSettings {
    // Generate x chunks infront of the player at all times
    pub chunk_generation_distance: usize,
}

impl Default for ChunkGenSettings {
    fn default() -> Self {
        Self {
            chunk_generation_distance: 10,
        }
    }
}

impl LandscapeController {
    pub fn new(settings: LandscapeControllerSettings) -> Self {
        Self {
            settings,
            chunks: HashMap::new(),
        }
    }

    pub fn world_position_to_tile(&self, position: Vec3) -> Tile {
        // multiply by -1 because chunks generate towards neg z, but tile positions are positive towards neg z
        let z_position = -1.0 * position.z
            / (self.settings.chunk_mesh_settings.verts_length as f32
                * self.settings.chunk_mesh_settings.vert_space_z);

        // Floor to make Tiles be 0 indexed.
        z_position.floor() as usize
    }

    pub fn tile_to_world_position(&self, tile: Tile) -> Vec3 {
        Vec3::new(
            self.settings.chunk_mesh_settings.vert_space_x
                * self.settings.chunk_mesh_settings.verts_width as f32
                * -0.5,
            0.0,
            -(tile as f32 * self.settings.chunk_mesh_settings.vert_space_z),
        )
    }
}
