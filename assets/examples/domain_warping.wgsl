#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_functions::{
    get_world_from_local,
    mesh2d_position_local_to_clip,
}
#import noisy_bevy::fbm_simplex_2d_seeded

struct NoiseMaterial {
    num_warps: i32
}

@group(2) @binding(0) var<uniform> material: NoiseMaterial;

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) object_position: vec2<f32>,
};


fn fbm_simplex_2d_warp(pos_initial: vec2<f32>, warp_iterations: i32, warp_scale: vec2<f32>, falloff: f32) -> f32 {
    let octaves = 8;
    let lacunarity = 2.7;
    let gain = 0.48;
    let seed_x = 324.0;
    let seed_y = 871.0;

    var scale = 1.0;
    var pos = pos_initial;

    for (var i: i32 = 0; i < warp_iterations; i++) {
        pos.x += scale * warp_scale.x * fbm_simplex_2d_seeded(pos, octaves, lacunarity, gain, seed_x);
        pos.y += scale * warp_scale.y * fbm_simplex_2d_seeded(pos, octaves, lacunarity, gain, seed_y);
        scale *= falloff;
    }

    return fbm_simplex_2d_seeded(pos, octaves, lacunarity, gain, seed_x);
}

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let model = get_world_from_local(vertex.instance_index);
    out.clip_position = mesh2d_position_local_to_clip(model, vec4<f32>(vertex.position, 1.0));
    out.object_position = vertex.position.xy;
    return out;
}

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let num_warps = 2;
    let frequency_scale = 0.001;
    var position = in.object_position * frequency_scale;
    position.x += globals.time * 0.006;
    
    var value = fbm_simplex_2d_warp(
        position,
        material.num_warps,
        vec2(0.4, 0.3),
        0.2,
    );
    value = (value + .3) / 2.0;

    return vec4(value, value, value, 1.);
}
