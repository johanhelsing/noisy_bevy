#import noisy_bevy::fbm_simplex_2d_seeded
#import bevy_sprite::mesh2d_functions::{
    get_world_from_local,
    mesh2d_position_local_to_clip,
}

struct AsteroidMaterial {
    params: vec4<f32>
}

@group(2) @binding(0) var<uniform> material: AsteroidMaterial;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) object_position: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    // Project the world position of the mesh into screen position
    let model = get_world_from_local(vertex.instance_index);
    out.clip_position = mesh2d_position_local_to_clip(model, vec4<f32>(vertex.position, 1.0));
    out.object_position = vertex.position.xy;
    return out;
}

@fragment
fn fragment(
    vertex_output: VertexOutput,
) -> @location(0) vec4<f32> {
    var p = vertex_output.object_position;
    let params = material.params;
    let freq_scale = params.x;
    let amp_scale = params.y;
    let radius = params.z;
    let seed = params.w;

    let r = sqrt(p.x * p.x + p.y * p.y);
    let d = r - radius;

    // smooth noise same as used on cpu...
    // let n = simplex_noise_2d(p * freq_scale) * amp_scale;

    // ...or add some extra turbulence to the "atmosphere"
    let n = fbm_simplex_2d_seeded(p * freq_scale, 7, 2.0, 0.5, seed) * amp_scale;

    var v = d - n;
    v = pow(-v * 0.1 + 0.3, 2.1);

    return vec4(v, v, v, 1.);
}