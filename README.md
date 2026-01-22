# noisy_bevy

[![crates.io](https://img.shields.io/crates/v/noisy_bevy.svg)](https://crates.io/crates/noisy_bevy)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
[![docs.rs](https://img.shields.io/docsrs/noisy_bevy)](https://docs.rs/noisy_bevy)

Simple stupid noise primitives for glam (`Vec2`, `Vec3`) and WGSL.

- Integrates with Bevy seamlessly
- Same results on the CPU and GPU (not bit-level perfect, though)

## Features

- [`simplex_noise_2d`]
- [`simplex_noise_2d_seeded`]
- [`simplex_noise_3d`]
- [`simplex_noise_3d_seeded`]
- [`fbm_simplex_2d`]
- [`fbm_simplex_2d_seeded`]
- [`fbm_simplex_3d`]
- [`fbm_simplex_3d_seeded`]
- [`worley_2d`]

![screenshot of an asteroid generated on the gpu and cpu](https://s3.johanhelsing.studio/dump/noisy_asteroid.png)

## Usage

### From Rust

Zero initialization, just call the noise functions:

```rust
use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d;

let p = Vec2::new(12.3, 45.6);
let value = simplex_noise_2d(p);
```

### From WGSL shaders

First add the plugin to the Bevy app:

```rust ignore
App::new()
    .add_plugins(NoisyShaderPlugin)
```

Then use it in your shaders:

```wgsl
#import noisy_bevy::simplex_noise_2d

// ...

let p = vec2(12.3, 45.6);
let value = simplex_noise_2d(p);
```

See the [`asteroids example`](https://github.com/johanhelsing/noisy_bevy/blob/main/examples/asteroids.rs), for an example that uses noise to procedurally generate a tilemap on the CPU and a matching background in a wgsl shader.

## Bevy Version Support

The `main` branch targets the latest bevy release.

|bevy|noisy\_bevy|
|----|-----------|
|0.18| 0.13, main|
|0.17| 0.11      |
|0.16| 0.9, 0.10 |
|0.15| 0.8       |
|0.14| 0.7       |
|0.13| 0.6       |
|0.12| 0.5       |
|0.11| 0.4       |
|0.10| 0.3       |
|0.9 | 0.2       |
|0.8 | 0.1       |

## License

MIT

The original simplex noise source is MIT-only, however all changes made by me or PRs to this repo are also available under Apache-2.0.

## Acknowledgments

The noise primitives are ports/copies of these

- <https://github.com/stegu/psrdnoise>
- <https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39>
- <https://github.com/bevy-interstellar/wgsl_noise>

## Contributions

PRs welcome!
