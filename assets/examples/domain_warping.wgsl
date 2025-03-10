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

const max_warp_iterations: i32 = 5;
const octaves = 8;
const lacunarity = 2.9;
const gain = 0.4;
const seed_x = 324.0;
const seed_y = 871.0;

fn fbm_simplex_2d_warp_positions(pos_initial: vec2<f32>, warp_iterations: i32, warp_scale: vec2<f32>, falloff: f32) -> array<vec2f, max_warp_iterations> {
    var scale = 1.0;
    var positions = array<vec2f, max_warp_iterations>();
    var pos = pos_initial;

    for (var i: i32 = 0; i < warp_iterations; i++) {
        pos.x += scale * warp_scale.x * fbm_simplex_2d_seeded(pos, octaves, lacunarity, gain, seed_x);
        pos.y += scale * warp_scale.y * fbm_simplex_2d_seeded(pos, octaves, lacunarity, gain, seed_y);
        positions[i] = pos;
        scale *= falloff;
    }

    return positions;
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
    let num_warps = 3;
    let frequency_scale = 0.005;
    var position = in.object_position * frequency_scale;
    position.x += globals.time * 0.0005;
    
    let warped_positions = fbm_simplex_2d_warp_positions(
        position,
        material.num_warps,
        vec2(0.3, 0.3),
        0.3,
    );

    var value = fbm_simplex_2d_seeded(warped_positions[2], octaves, lacunarity, gain, seed_x);

    // Palette 1 - blue
    // let base_color_a = vec4(223./255., 242./255., 235./255., 1.);
    // let base_color_b = vec4(185./255., 229./255., 232./255., 1.);
    // let base_color_c = vec4(122./255., 178./255., 211./255., 1.);
    // let base_color_d = vec4(74./255., 98./255., 138./255., 1.);

    // Palette 2 - red
    // let base_color_a = vec4(9./255., 18./255., 44./255., 1.);
    // let base_color_b = vec4(135./255., 35./255., 65./255., 1.);
    // let base_color_c = vec4(190./255., 49./255., 68./255., 1.);
    // let base_color_d = vec4(225./255., 117./255., 100./255., 1.);

    // Palette 3 - green
    let base_color_a = vec4(26./255., 26./255., 25./255., 1.);
    let base_color_b = vec4(49./255., 81./255., 30./255., 1.);
    let base_color_c = vec4(133./255., 159./255., 61./255., 1.);
    let base_color_d = vec4(246./255., 252./255., 223./255., 1.);

    let color1 = mix(
        base_color_a,
        base_color_d,
        smoothstep(0.0, 1.0, value)
    );

    let color2 = mix(
        color1,
        base_color_b,
        smoothstep(0.0, 1.0, length(warped_positions[0]))
    );

    let color3 = mix(
        color2,
        base_color_c,
        warped_positions[1].y
    );

    return color3;
}
