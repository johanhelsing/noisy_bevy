#import bevy_sprite::mesh2d_functions::{
    get_world_from_local,
    mesh2d_position_local_to_clip,
}
#import noisy_bevy::worley_2d

struct NoiseMaterial {
    frequency_scale: f32,
    amplitude_scale: f32,
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
    // Project the world position of the mesh to screen position
    let model = get_world_from_local(vertex.instance_index);
    out.clip_position = mesh2d_position_local_to_clip(model, vec4<f32>(vertex.position, 1.0));
    out.object_position = vertex.position.xy;
    return out;
}

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let position = in.object_position * material.frequency_scale;
    let jitter = 1.0;
    let worley = worley_2d(position, jitter) * material.amplitude_scale;
    
    let distance = worley.x; 

    return vec4(distance, distance, distance, 1.);
}
