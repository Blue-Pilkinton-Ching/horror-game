use bevy::math::ops::sin;

use crate::plugins::landscape::landscape::chunk::ChunkNoiseSettings;

pub fn sample_noise(x: f32, z: f32, noise_settings: ChunkNoiseSettings) -> f32 {
    return sin(z * noise_settings.frequency) * noise_settings.noise_scale;
}
