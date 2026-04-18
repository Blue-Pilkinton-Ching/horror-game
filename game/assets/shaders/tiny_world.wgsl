#import bevy_pbr::{
    mesh_functions,
    forward_io::{Vertex, VertexOutput},
    view_transformations::position_world_to_clip
}

struct TinyWorldParams {
    curve_strength: f32,
    player_position_z: f32,
    falloff_rate: f32,
    origin_offset: f32,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> params : TinyWorldParams;

@vertex
fn vertex(in: Vertex) -> VertexOutput {
    /* --- Model → World transform (Bevy helper) ------------------- */
    var model = mesh_functions::get_world_from_local(in.instance_index);
    var world_pos = mesh_functions::mesh_position_local_to_world(
        model,
        vec4<f32>(in.position, 1.0)
    );

    var rel_z = world_pos.z - params.player_position_z;
    var x = max(abs(rel_z) - params.origin_offset, 0.0);
    var rotation_amount = params.curve_strength * (1.0 - exp(-x * x * params.falloff_rate)) * -sign(rel_z);

    var new_z = (rel_z * cos(rotation_amount)) - (world_pos.y * sin(rotation_amount));
    var new_y = (rel_z * sin(rotation_amount)) + (world_pos.y * cos(rotation_amount));

    world_pos.z = new_z + params.player_position_z;
    world_pos.y = new_y;

    /* --- Fill the required output struct ------------------------- */
    var out: VertexOutput;
    out.world_position = world_pos;
    out.position = position_world_to_clip(world_pos.xyz);

    /* Pass-through you may need later */
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        in.normal, in.instance_index
    );
    out.uv = in.uv;

    return out;
}