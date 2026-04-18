use bevy::math::Vec2;
use noise::NoiseFn;

use noise::{NoiseFn, Perlin, Seedable};

let perlin = Perlin::new(2);
let val = perlin.get([42.4, 37.7, 2.8]);

#[derive(Clone)]
pub struct NoiseSettings {
    pub offset: Vec2,
    pub seed: i32,
    pub layers: NoiseFn,
}

// pub struct NoiseLayer {
//     // apply a function to blend this layer with the one before
//     // First param is the bottom later, second one is the top
//     blend_function: Fn(f32, f32) -> f32,
//     layer_type: NoiseLayerSettings,
//     next_layer: Option<NoiseLayer>,
// }

// #[derive(PartialEq)]
// pub enum NoiseLayerSettings {
//     Fbm(FbmLayerSettings),
//     Gradient(GradientLayerSettings),
//     Custom(CustomNoiseSettings),
// }

// pub struct FbmLayerSettings {
//     pub freq: f32,
//     pub lacunarity: f32,
//     pub gain: f32,
//     pub octaves: u8,
// }

// pub struct GradientLayerSettings {
//     pub freq: f32,
// }

// pub struct CustomNoiseSettings {
//     pub noise_function: CustomNoiseLayerFn,
// }

// Takes a noise position as a Vec2 and returns the sampled noise
pub type CustomNoiseLayerFn = Fn(Vec2) -> f32;

// pub fn sample_noise_grid(noise_settings: NoiseSettings, size: U32Vec2, offset: Vec2) -> Vec<f32> {
//     sample_noise_grid_layer(
//         noise_settings.layers,
//         size,
//         offset,
//         noise_settings.seed,
//         None,
//     )
// }

// fn sample_noise_grid_layer(
//     layer: NoiseLayer,
//     size: U32Vec2,
//     offset: Vec2,
//     seed: i32,
//     running_sample: Option<Vec<f32>>,
// ) -> Vec<f32> {
//     let new_sample = match layer.layer_type {
//         NoiseLayerSettings::Fbm(settings) => {
//             NoiseBuilder::fbm_2d_offset(offset.x, size.x, offset.y, size.y)
//                 .with_freq(settings.freq)
//                 .with_gain(settings.gain)
//                 .with_lacunarity(settings.lacunarity)
//                 .with_octaves(settings.octaves)
//                 .with_seed(seed)
//                 .generate()
//                 .0
//         }
//         NoiseLayerSettings::Gradient(settings) => {
//             NoiseBuilder::gradient_2d_offset(offset.x, size.x, offset.y, size.y)
//                 .with_freq(settings.freq)
//                 .with_seed(seed)
//                 .generate()
//                 .0
//         }
//         NoiseLayerSettings::Custom(settings) => (0..(settings.size.x * settings.size.y))
//             .map(|i| {
//                 let x = i % settings.size.x;
//                 let y = settings.size.y - (x * settings.size.x);
//                 settings.noise_function(Vec2::new(x as f32, y as f32))
//             })
//             .collect::<Vec<f32>>(),
//     };

//     let computed_sample = match running_sample {
//         None => new_sample,
//         Some(running_sample) => layer.blend_function(running_sample, new_sample),
//     };

//     match layer.next_layer {
//         None => computed_sample,
//         Some(next_layer) => {
//             sample_noise_grid_layer(next_layer, size, offset, seed, Some(computed_sample))
//         }
//     }
// }

pub fn sample_noise(pos: Vec2, noise_settings: NoiseSettings) -> f32 {
    sample_noise_layer(noise_settings.layers, pos, None)
}

fn sample_noise_layer(layer: NoiseLayer, pos: Vec2, running_sample: Option<f32>) -> f32 {
    let new_sample: f32 = match layer.layer_type {
        NoiseLayerSettings::Fbm(settings) => NoiseBuilder::fbm_2d_offset(pos.x, 1, pos.y, 1)
            .with_freq(settings.freq)
            .with_gain(settings.gain)
            .with_lacunarity(settings.lacunarity)
            .with_octaves(settings.octaves)
            .with_seed(seed)
            .generate()
            .0
            .get(0)
            .cloned()
            .expect("Sampled noise vector does not contain a value"),
        NoiseLayerSettings::Gradient(settings) => {
            NoiseBuilder::gradient_2d_offset(pos.x, 1, pos.y, 1)
                .with_freq(settings.freq)
                .with_seed(seed)
                .generate()
                .0
                .get(0)
                .cloned()
                .expect("Sampled noise vector does not contain a value")
        }
        NoiseLayerSettings::Custom(settings) => settings.noise_function(pos),
    };

    let computed_sample = match running_sample {
        None => new_sample,
        Some(running_sample) => layer.blend_function(running_sample, new_sample),
    };

    match layer.next_layer {
        None => computed_sample,
        Some(next_layer) => sample_noise_layer(next_layer, pos, Some(computed_sample)),
    }
}
