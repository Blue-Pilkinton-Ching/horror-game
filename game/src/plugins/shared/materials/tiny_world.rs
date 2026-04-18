use bevy::{
    pbr::MaterialExtension,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
};

#[derive(ShaderType, Copy, Default, Debug, Clone)]
pub struct TinyWorldMaterialParams {
    pub amplitude: f32,
    pub frequency: f32,
    pub speed: f32,
    pub phase: f32,
    pub time: f32,
}

#[derive(AsBindGroup, Asset, TypePath, Default, Debug, Clone)]
pub struct TinyWorldMaterial {
    #[uniform(100)]
    pub params: TinyWorldMaterialParams,
}

impl MaterialExtension for TinyWorldMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/fish_wobble.wgsl".into()
    }
}
