use bevy::prelude::*;

use crate::plugins::landscape::landscape::{
    chunk::ChunkMeshSettings,
    controller::{ChunkGenSettings, LandscapeController, LandscapeControllerSettings},
};

mod landscape_startup;
mod landscape_update;

mod landscape;

pub struct LandscapePlugin;

impl Plugin for LandscapePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Startup, landscape_startup::startup)
            .add_systems(Update, landscape_update::start_generating_new_chunks)
            .add_systems(Update, landscape_update::finish_generating_new_chunks)
            .insert_resource(LandscapeController::new(LandscapeControllerSettings {
                chunk_gen_settings: ChunkGenSettings::default(),
                chunk_mesh_settings: ChunkMeshSettings {
                    verts_width: 20,
                    verts_length: 20,
                    vert_space_x: 1.0,
                    vert_space_z: 1.0,
                },
            }));
    }
}
