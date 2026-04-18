use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
};

#[derive(ShaderType, Copy, Debug, Clone)]
pub struct TinyWorldParams {
    pub curve_strength: f32,
    pub player_position_z: f32,
    pub falloff_rate: f32,
    pub origin_offset: f32,
}

impl Default for TinyWorldParams {
    fn default() -> Self {
        Self {
            curve_strength: 1.2,
            falloff_rate: 0.000001,
            origin_offset: 200.0,
            player_position_z: 0.0,
        }
    }
}

#[derive(AsBindGroup, Asset, TypePath, Default, Debug, Clone)]
pub struct TinyWorldExt {
    #[uniform(100)]
    pub params: TinyWorldParams,
}

impl MaterialExtension for TinyWorldExt {
    fn vertex_shader() -> ShaderRef {
        "shaders/tiny_world.wgsl".into()
    }
}

pub type TinyWorldMaterialExt = ExtendedMaterial<StandardMaterial, TinyWorldExt>;
