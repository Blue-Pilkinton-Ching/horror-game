use std::sync::Arc;

use bevy::math::Vec2;
use noise::NoiseFn;

#[derive(Clone)]
pub struct NoiseSettings {
    pub noise_x_freq_multiplier: f64,
    pub noise_fn: Arc<dyn NoiseFn<f64, 2> + Send + Sync>,
}

pub fn sample_noise(pos: Vec2, noise_fn: &dyn NoiseFn<f64, 2>) -> f32 {
    noise_fn.get([pos.x as f64, pos.y as f64]) as f32
}
