use std::sync::Arc;

use bevy::prelude::*;
use noise::MultiFractal;
use noise::{Constant, Fbm, Multiply, Perlin};

use crate::plugins::landscape::landscape::{
    chunk::ChunkMeshSettings,
    controller::{ChunkGenSettings, LandscapeController, LandscapeControllerSettings},
    noise::NoiseSettings,
};

pub mod landscape;
mod landscape_startup;
mod landscape_update;

pub struct LandscapePlugin;

const GROUND_SEED: u32 = 1;

impl Plugin for LandscapePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Startup, landscape_startup::startup)
            .add_systems(Update, landscape_update::start_generating_new_chunks)
            .add_systems(Update, landscape_update::finish_generating_new_chunks)
            .insert_resource(LandscapeController::new(LandscapeControllerSettings {
                chunk_gen: ChunkGenSettings::default(),
                chunk_mesh: ChunkMeshSettings {
                    // 25 meters per chunk (0.5 * 50)
                    verts_width: 500,
                    verts_length: 500,
                    // Two tris per meter
                    vert_space_x: 0.1,
                    vert_space_z: 0.1,
                },
                noise: NoiseSettings {
                    noise_x_freq_multiplier: 0.05,
                    noise_fn: Arc::new(Multiply::new(
                        Fbm::<Perlin>::new(GROUND_SEED)
                            .set_frequency(0.005)
                            .set_octaves(3),
                        Constant::new(20.0),
                    )),
                },
            }));
    }
}
