#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::{
    asset::load_internal_asset,
    math::{vec2, vec3, vec4, Vec2Swizzles, Vec3Swizzles, Vec4Swizzles},
    prelude::*,
};

/// Adds noise library as a wgsl import
///
/// General functionality can be imported through:
///
/// ```wgsl
/// #import noisy_bevy
/// ```
pub struct NoisyShaderPlugin;

impl Plugin for NoisyShaderPlugin {
    fn build(&self, app: &mut App) {
        // workaround: embedded_asset is broken in bevy 0.12.0
        load_internal_asset!(
            app,
            NOISY_SHADER_HANDLE,
            "../assets/noisy_bevy.wgsl",
            Shader::from_wgsl
        );
    }
}

const NOISY_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(224136012015454690045205738992444526155);

fn permute_3(x: Vec3) -> Vec3 {
    (((x * 34.) + 1.) * x) % Vec3::splat(289.)
}

// MIT License. © Ian McEwan, Stefan Gustavson, Munrocket, Johan Helsing
/// Simplex noise in two dimensions
pub fn simplex_noise_2d(v: Vec2) -> f32 {
    const C: Vec4 = vec4(
        0.211_324_87,  // (3.0 - sqrt(3.0)) / 6.0
        0.366_025_42,  // 0.5 * (sqrt(3.0) - 1.0)
        -0.577_350_26, // -1.0 + 2.0 * C.x
        1. / 41.,
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
    m *= 1.792_842_9 - 0.853_734_73 * (a0 * a0 + h * h);
    let g = vec3(
        a0.x * x0.x + h.x * x0.y,
        a0.y * x12.x + h.y * x12.y,
        a0.z * x12.z + h.z * x12.w,
    );
    130. * Vec3::dot(m, g)
}

// MIT License. © Ian McEwan, Stefan Gustavson, Munrocket, Johan Helsing
/// Simplex noise in two dimensions
pub fn simplex_noise_2d_seeded(v: Vec2, seed: f32) -> f32 {
    const C: Vec4 = vec4(
        0.211_324_87,  // (3.0 - sqrt(3.0)) / 6.0
        0.366_025_42,  // 0.5 * (sqrt(3.0) - 1.0)
        -0.577_350_26, // -1.0 + 2.0 * C.x
        1. / 41.,
    );

    // first corner
    let mut i: Vec2 = (v + Vec2::dot(v, C.yy())).floor();
    let x0 = v - i + Vec2::dot(i, C.xx());

    // other corners
    let i1: Vec2 = if x0.x > x0.y {
        vec2(1., 0.)
    } else {
        vec2(0., 1.)
    };
    let x12: Vec4 = x0.xyxy() + C.xxzz() - vec4(i1.x, i1.y, 0., 0.);

    // permutations
    i %= Vec2::splat(289.);

    let mut p = permute_3(permute_3(i.y + vec3(0., i1.y, 1.)) + i.x + vec3(0., i1.x, 1.));
    p = permute_3(p + Vec3::splat(seed));

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

    // gradients: 41 points uniformly over a line, mapped onto a diamond
    // the ring size, 17*17 = 289, is close to a multiple of 41 (41*7 = 287)
    let x = 2. * (p * C.www()).fract() - 1.;
    let h = x.abs() - 0.5;
    let ox = (x + 0.5).floor();
    let a0 = x - ox;

    // normalize gradients implicitly by scaling m
    // approximation of: m *= inversesqrt(a0 * a0 + h * h);
    m *= 1.792_842_9 - 0.853_734_73 * (a0 * a0 + h * h);
    let g = vec3(
        a0.x * x0.x + h.x * x0.y,
        a0.y * x12.x + h.y * x12.y,
        a0.z * x12.z + h.z * x12.w,
    );

    // compute final noise value at P
    130. * Vec3::dot(m, g)
}

fn permute_4(x: Vec4) -> Vec4 {
    ((x * 34. + 1.) * x) % Vec4::splat(289.)
}

fn taylor_inv_sqrt_4(r: Vec4) -> Vec4 {
    1.792_842_9 - 0.853_734_73 * r
}

#[inline]
fn step_4(edge: Vec4, x: Vec4) -> Vec4 {
    let b = Vec4::cmple(edge, x);
    Vec4::select(b, Vec4::ONE, Vec4::ZERO)
}

#[inline]
fn step_3(edge: Vec3, x: Vec3) -> Vec3 {
    let b = Vec3::cmple(edge, x);
    Vec3::select(b, Vec3::ONE, Vec3::ZERO)
}

// MIT License. © Ian McEwan, Stefan Gustavson, Munrocket, Johan Helsing
/// Simplex noise in three dimensions
pub fn simplex_noise_3d(v: Vec3) -> f32 {
    const C: Vec2 = vec2(1. / 6., 1. / 3.);
    const D: Vec4 = vec4(0., 0.5, 1., 2.);

    // first corner
    let mut i = (v + Vec3::dot(v, C.yyy())).floor();
    let x0 = v - i + Vec3::dot(i, C.xxx());

    // other corners
    let g = step_3(x0.yzx(), x0.xyz());
    let l = 1. - g;
    let i1 = Vec3::min(g.xyz(), l.zxy());
    let i2 = Vec3::max(g.xyz(), l.zxy());

    // x0 = x0 - 0. + 0. * C
    let x1 = x0 - i1 + 1. * C.xxx();
    let x2 = x0 - i2 + 2. * C.xxx();
    let x3 = x0 - 1. + 3. * C.xxx();

    // permutations
    i %= Vec3::splat(289.);
    let p = permute_4(
        permute_4(permute_4(i.z + vec4(0., i1.z, i2.z, 1.)) + i.y + vec4(0., i1.y, i2.y, 1.))
            + i.x
            + vec4(0., i1.x, i2.x, 1.),
    );

    // gradients (NxN points uniformly over a square, mapped onto an octahedron)
    let n_ = 1. / 7.; // N=7
    let ns = n_ * D.wyz() - D.xzx();

    let j = p - 49. * (p * ns.z * ns.z).floor(); // mod(p, N*N)

    let x_ = (j * ns.z).floor();
    let y_ = (j - 7. * x_).floor(); // mod(j, N)

    let x = x_ * ns.x + ns.yyyy();
    let y = y_ * ns.x + ns.yyyy();
    let h = 1. - x.abs() - y.abs();

    let b0 = vec4(x.x, x.y, y.x, y.y);
    let b1 = vec4(x.w, x.w, y.z, y.w);

    let s0 = b0.floor() * 2. + 1.;
    let s1 = b1.floor() * 2. + 1.;
    let sh = -step_4(h, Vec4::splat(0.));

    let a0 = b0.xzyw() + s0.xzyw() * sh.xxyy();
    let a1 = b1.xzyw() + s1.xzyw() * sh.zzww();

    let mut p0 = a0.xy().extend(h.x);
    let mut p1 = a0.zw().extend(h.y);
    let mut p2 = a1.xy().extend(h.z);
    let mut p3 = a1.zw().extend(h.w);

    // normalize gradients
    let norm = taylor_inv_sqrt_4(vec4(
        Vec3::dot(p0, p0),
        Vec3::dot(p1, p1),
        Vec3::dot(p2, p2),
        Vec3::dot(p3, p3),
    ));
    p0 *= norm.x;
    p1 *= norm.y;
    p2 *= norm.z;
    p3 *= norm.w;

    // mix final noise value
    let mut m = 0.6
        - vec4(
            Vec3::dot(x0, x0),
            Vec3::dot(x1, x1),
            Vec3::dot(x2, x2),
            Vec3::dot(x3, x3),
        );
    m = Vec4::max(m, Vec4::ZERO);
    m *= m;
    42. * Vec4::dot(
        m * m,
        vec4(
            Vec3::dot(p0, x0),
            Vec3::dot(p1, x1),
            Vec3::dot(p2, x2),
            Vec3::dot(p3, x3),
        ),
    )
}

/// Fractional brownian motion (fbm) based on 2d simplex noise
pub fn fbm_simplex_2d(pos: Vec2, octaves: usize, lacunarity: f32, gain: f32) -> f32 {
    let mut sum = 0.;
    let mut amplitude = 1.;
    let mut frequency = 1.;

    for _ in 0..octaves {
        sum += simplex_noise_2d(pos * frequency) * amplitude;
        amplitude *= gain;
        frequency *= lacunarity;
    }

    sum
}

/// Fractional brownian motion (fbm) based on seeded 2d simplex noise
pub fn fbm_simplex_2d_seeded(
    pos: Vec2,
    octaves: usize,
    lacunarity: f32,
    gain: f32,
    seed: f32,
) -> f32 {
    let mut sum = 0.;
    let mut amplitude = 1.;
    let mut frequency = 1.;

    for _ in 0..octaves {
        sum += simplex_noise_2d_seeded(pos * frequency, seed) * amplitude;
        amplitude *= gain;
        frequency *= lacunarity;
    }

    sum
}

const MAX_WARP_ITERATIONS: usize = 4;

/// The return value of the domain warping function.
#[derive(Copy, Clone)]
pub struct WarpResult {
    /// The final noise value
    pub noise_value: f32,
    /// The history of warped coordinates, where positions[0] is the last iteration, positions[1] is second to last, etc.
    /// Can be useful for mixing colors.
    pub positions: [Vec2; MAX_WARP_ITERATIONS],
}

/// A technique that distorts the position before feeding it to the noise
/// inspired by https://iquilezles.org/articles/warp/
#[allow(clippy::too_many_arguments)]
pub fn fbm_simplex_2d_warp_seeded(
    pos_initial: Vec2,
    octaves: usize,
    lacunarity: f32,
    gain: f32,
    seed: f32,
    warp_iterations: usize,
    warp_scale: Vec2,
    falloff: f32,
) -> WarpResult {
    let mut pos = pos_initial;
    let mut scale = 1.0;
    let mut positions = [Vec2::ZERO; MAX_WARP_ITERATIONS];

    for i in 0..warp_iterations {
        pos.x += scale * warp_scale.x * fbm_simplex_2d_seeded(pos, octaves, lacunarity, gain, seed);
        pos.y += scale * warp_scale.y * fbm_simplex_2d_seeded(pos, octaves, lacunarity, gain, seed);

        // Store positions in reverse order (last iteration at index 0)
        let index = MAX_WARP_ITERATIONS - 1 - i;
        if index < MAX_WARP_ITERATIONS {
            positions[index] = pos;
        }

        scale *= falloff;
    }

    let noise_value = fbm_simplex_2d_seeded(positions[3], octaves, lacunarity, gain, seed);

    WarpResult {
        noise_value,
        positions,
    }
}

/// Fractional brownian motion (fbm) based on 3d simplex noise
pub fn fbm_simplex_3d(pos: Vec3, octaves: usize, lacunarity: f32, gain: f32) -> f32 {
    let mut sum = 0.;
    let mut amplitude = 1.;
    let mut frequency = 1.;

    for _ in 0..octaves {
        sum += simplex_noise_3d(pos * frequency) * amplitude;
        amplitude *= gain;
        frequency *= lacunarity;
    }

    sum
}

/// Cellular noise
pub fn worley_2d(pos: Vec2, jitter: f32) -> Vec2 {
    const K: f32 = 1.0 / 7.0;
    const KO: f32 = 3.0 / 7.0;

    // Determine the grid cell and fractional position
    let pi = pos.floor();
    let pf = pos.fract_gl();

    // Define offset indices for neighboring grid cells
    let oi = vec3(-1.0, 0.0, 1.0);
    let of_ = vec3(-0.5, 0.5, 1.5);

    // Permute the grid cell indices to get unique values for each cell
    let px = permute_3(pi.x + oi);
    let mut p = permute_3(px.x + pi.y + oi); // p11, p12, p13

    let mut ox = (p * K).fract_gl() - KO;
    let mut oy = (p * K).floor() % 7.0 * K - KO;
    let mut dx = pf.x + 0.5 + jitter * ox;
    let mut dy = pf.y - of_ + jitter * oy;
    let mut d1 = dx * dx + dy * dy;

    p = permute_3(px.y + pi.y + oi); // p21, p22, p23
    ox = (p * K).fract_gl() - KO;
    oy = ((p * K).floor() % 7.0) * K - KO;
    dx = pf.x - 0.5 + jitter * ox;
    dy = pf.y - of_ + jitter * oy;
    let mut d2 = dx * dx + dy * dy; // d21, d22, d23, squared

    p = permute_3(px.z + pi.y + oi); // p31, p32, p33
    ox = (p * K).fract_gl() - KO;
    oy = ((p * K).floor() % 7.0) * K - KO;
    dx = pf.x - 1.5 + jitter * ox;
    dy = pf.y - of_ + jitter * oy;
    let d3 = dx * dx + dy * dy; // d31, d32, d33, squared

    // Find the two smallest distances (F1 and F2)
    let d1a = d1.min(d2);
    d2 = d1.max(d2);
    d2 = d2.min(d3);
    d1 = d1a.min(d2);
    d2 = d1a.max(d2);

    if d1.x > d1.y {
        core::mem::swap(&mut d1.x, &mut d1.y);
    }

    if d1.x > d1.z {
        core::mem::swap(&mut d1.x, &mut d1.z);
    }

    d1.y = d1.y.min(d2.y);
    d1.z = d1.z.min(d2.z);
    d1.y = d1.y.min(d1.z);
    d1.y = d1.y.min(d2.x);

    vec2(d1.x.sqrt(), d1.y.sqrt())
}

#[cfg(test)]
mod test {
    use super::*;
    use insta::assert_debug_snapshot;

    fn sample_2d_fn(f: fn(Vec2) -> f32) -> Vec<f32> {
        let mut values = Vec::new();
        for x in -20..20 {
            let x = x as f32 / 10.;
            for y in -20..20 {
                let y = y as f32 / 10.;
                let v = f(vec2(x, y));
                values.push(v);
            }
        }
        values
    }

    fn sample_3d_fn(f: fn(Vec3) -> f32) -> Vec<f32> {
        let mut values = Vec::new();
        for x in -5..5 {
            let x = x as f32 / 10.;
            for y in -5..5 {
                let y = y as f32 / 10.;
                for z in -5..5 {
                    let z = z as f32 / 10.;
                    let v = f(vec3(x, y, z));
                    values.push(v);
                }
            }
        }
        values
    }

    #[test]
    fn simplex_2d_values_unchanged() {
        assert_debug_snapshot!(sample_2d_fn(simplex_noise_2d));
    }

    #[test]
    fn simplex_2d_seeded_values_unchanged() {
        assert_debug_snapshot!(sample_2d_fn(|p| simplex_noise_2d_seeded(p, 0.0)));
        assert_debug_snapshot!(sample_2d_fn(|p| simplex_noise_2d_seeded(p, 123.0)));
    }

    #[test]
    fn fbm_2d_warp_seeded_values_unchanged() {
        assert_debug_snapshot!(sample_2d_fn(|p| {
            fbm_simplex_2d_warp_seeded(p, 10, 2.9, 0.4, 324.0, 4, vec2(0.4, 0.4), 0.1).noise_value
        }));
    }

    #[test]
    fn simplex_3d_values_unchanged() {
        assert_debug_snapshot!(sample_3d_fn(simplex_noise_3d));
    }

    #[test]
    fn fbm_2d_values_unchanged() {
        assert_debug_snapshot!(sample_2d_fn(|p| { fbm_simplex_2d(p, 5, 2.0, 0.5) }));
    }

    #[test]
    fn fbm_2d_seeded_values_unchanged() {
        assert_debug_snapshot!(sample_2d_fn(|p| {
            fbm_simplex_2d_seeded(p, 5, 2.0, 0.5, 0.0)
        }));
        assert_debug_snapshot!(sample_2d_fn(|p| {
            fbm_simplex_2d_seeded(p, 5, 2.0, 0.5, 123.0)
        }));
    }

    #[test]
    fn fbm_3d_values_unchanged() {
        assert_debug_snapshot!(sample_3d_fn(|p| { fbm_simplex_3d(p, 5, 2.0, 0.5) }));
    }

    #[test]
    fn worley_2d_values_unchanged() {
        assert_debug_snapshot!({
            let mut values = Vec::new();
            for x in -20..20 {
                let x = x as f32 / 10.;
                for y in -20..20 {
                    let y = y as f32 / 10.;
                    let v = worley_2d(vec2(x, y), 1.0);
                    values.push(v);
                }
            }
            values
        });
    }
}
