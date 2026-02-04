#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_functions::{
    get_world_from_local,
    mesh2d_position_local_to_clip,
}
#import noisy_bevy::fbm_simplex_2d_warp_seeded

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

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let model = get_world_from_local(vertex.instance_index);
    out.clip_position = mesh2d_position_local_to_clip(model, vec4<f32>(vertex.position, 1.0));
    out.object_position = vertex.position.xy;
    return out;
}

const frequency_scale = 0.005;
const octaves = 10;
const lacunarity = 2.9;
const gain = 0.4;
const seed = 324.0;
const scale = vec2(0.4, 0.4);
const faloff = 0.1;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    var position = in.object_position * frequency_scale;
    position.x += globals.time * 0.001;
    
    let warp_result = fbm_simplex_2d_warp_seeded(
        position,
        octaves,
        lacunarity,
        gain,
        seed,
        material.num_warps,
        scale,
        faloff
    );

    // Palette 1 - frost
    // let base_color_a = vec4(74./255., 98./255., 138./255., 1.);
    // let base_color_b = vec4(185./255., 229./255., 232./255., 1.);
    // let base_color_c = vec4(122./255., 178./255., 211./255., 1.);
    // let base_color_d = vec4(223./255., 242./255., 235./255., 1.);

    // Palette 2 - bath bomb
    let base_color_a = vec4(9./255., 18./255., 44./255., 1.);
    let base_color_b = vec4(135./255., 35./255., 65./255., 1.);
    let base_color_c = vec4(190./255., 49./255., 68./255., 1.);
    let base_color_d = vec4(225./255., 117./255., 100./255., 1.);

    // Palette 3 - toxic
    // let base_color_a = vec4(26./255., 26./255., 25./255., 1.);
    // let base_color_b = vec4(49./255., 81./255., 30./255., 1.);
    // let base_color_c = vec4(133./255., 159./255., 61./255., 1.);
    // let base_color_d = vec4(246./255., 252./255., 223./255., 1.);

    let color1 = mix(
        base_color_a,
        base_color_d,
        smoothstep(0.0, 1.0, warp_result.noise_value)
    );

    let color2 = mix(
        color1,
        base_color_b,
        smoothstep(0.0, 3.0, length(warp_result.positions[0]))
    );

    let color3 = mix(
        color2,
        base_color_c,
        warp_result.positions[1].y
    );

    return color3;
}
