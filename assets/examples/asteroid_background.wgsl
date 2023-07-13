#import noisy_bevy fbm_simplex_2d_seeded
#import bevy_sprite::mesh2d_bindings mesh
#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput

struct AsteroidMaterial {
    params: vec4<f32>
}

@group(1) @binding(0)
var<uniform> material: AsteroidMaterial;

@fragment
fn fragment(
    vertex_output: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    // perf: better to do in vertex shader!
    var p = vertex_output.world_position.xy - vec2(mesh.model[3].x, mesh.model[3].y); // ignoring rotation
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