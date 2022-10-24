#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::{
    asset::HandleId,
    math::{vec2, vec3, vec4, Vec2Swizzles, Vec4Swizzles},
    prelude::*,
};

/// Adds noise library as a wgsl import
///
/// General functionality can be included through:
///
/// ```wgsl
/// #import noisy_bevy::prelude
/// ```
pub struct NoisyShaderPlugin;

impl Plugin for NoisyShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_shaders);
    }
}

fn load_shaders(mut shaders: ResMut<Assets<Shader>>) {
    let shader = Shader::from_wgsl(include_str!("../assets/simple_noise_prelude.wgsl"));
    let handle_id = HandleId::random::<Shader>();
    shaders.set_untracked(handle_id, shader);
}

fn permute_3(x: Vec3) -> Vec3 {
    (((x * 34.) + 1.) * x) % Vec3::splat(289.)
}

// MIT License. © Ian McEwan, Stefan Gustavson, Munrocket, Johan Helsing
/// Simplex noise in two dimensions
pub fn simplex_2d(v: Vec2) -> f32 {
    const C: Vec4 = vec4(
        0.211324865405187,
        0.366025403784439,
        -0.577350269189626,
        0.024390243902439,
    );
    let mut i: Vec2 = (v + Vec2::dot(v, C.yy())).floor();
    let x0 = v - i + Vec2::dot(i, C.xx());
    let i1: Vec2 = if x0.x > x0.y {
        vec2(1., 0.)
    } else {
        vec2(0., 1.)
    };
    let x12: Vec4 = x0.xyxy() + C.xxzz() - vec4(i1.x, i1.y, 0., 0.);
    i %= Vec2::splat(289.);
    let p = permute_3(permute_3(i.y + vec3(0., i1.y, 1.)) + i.x + vec3(0., i1.x, 1.));
    let mut m = Vec3::max(
        0.5 - vec3(
            Vec2::dot(x0, x0),
            Vec2::dot(x12.xy(), x12.xy()),
            Vec2::dot(x12.zw(), x12.zw()),
        ),
        Vec3::splat(0.),
    );
    m *= m;
    m *= m;
    let x = 2. * (p * C.www()).fract() - 1.;
    let h = x.abs() - 0.5;
    let ox = (x + 0.5).floor();
    let a0 = x - ox;
    m *= 1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h);
    let g = vec3(
        a0.x * x0.x + h.x * x0.y,
        a0.y * x12.x + h.y * x12.y,
        a0.z * x12.z + h.z * x12.w,
    );
    130. * Vec3::dot(m, g)
}
