#import bevy_pbr::{
    mesh_functions,
    forward_io::{Vertex, VertexOutput},
    view_transformations::position_world_to_clip
}

struct WobbleParams {
    amplitude : f32,
    frequency : f32,
    speed     : f32,
    phase     : f32,
    time      : f32,
};

@group(2) @binding(100)
var<uniform> params : WobbleParams;

@vertex
fn vertex(in: Vertex) -> VertexOutput {
    /* --- Model → World transform (Bevy helper) ------------------- */
    var model = mesh_functions::get_world_from_local(in.instance_index);
    var world_pos = mesh_functions::mesh_position_local_to_world(
        model,
        vec4<f32>(in.position, 1.0)
    );

    /* --- Apply wobble in world space ----------------------------- */
    let phase = in.position.x * params.frequency +
                (params.time + params.phase) * params.speed;
    world_pos.z += sin(phase) * params.amplitude;

    /* --- Fill the required output struct ------------------------- */
    var out : VertexOutput;
    out.world_position = world_pos;
    out.position = position_world_to_clip(world_pos.xyz);

    /* Pass-through you may need later */
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        in.normal, in.instance_index
    );
    out.uv = in.uv;

    return out;
}