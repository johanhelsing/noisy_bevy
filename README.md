# noisy_bevy

[![crates.io](https://img.shields.io/crates/v/noisy_bevy.svg)](https://crates.io/crates/noisy_bevy)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
[![docs.rs](https://img.shields.io/docsrs/noisy_bevy)](https://docs.rs/noisy_bevy)

Simple stupid noise primitives for glam types (`Vec2`, `Vec3`) and wgsl.

Main motivations are:

- ergonomic usage with Bevy
- same results on rust and wgsl (not bit-level perfect, though)

![screenshot of an asteroid generated on the gpu and cpu](https://s3.johanhelsing.studio/dump/noisy_asteroid.png)

## Implemented noise primitives:

- [`simplex_noise_2d`]
- [`simplex_noise_2d_seeded`]
- [`simplex_noise_3d`]
- [`fbm_simplex_2d`]
- [`fbm_simplex_2d_seeded`]
- [`fbm_simplex_3d`]

## Usage

### From rust

Zero initialization, just call the noise functions:

```rust
use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d;

let p = Vec2::new(12.3, 45.6);
let value = simplex_noise_2d(p);
```

### From wgsl shaders

First add the plugin to the Bevy app:

```rust ignore
App::new()
    .add_plugin(NoisyShaderPlugin)
```

And import it and use it in your shaders, with the same API as on the CPU-side:

```wgsl
#import noisy_bevy::prelude

// ...

let p = vec2(12.3, 45.6);
let value = simplex_noise_2d(p);
```

See the [`asteroids example`](https://github.com/johanhelsing/noisy_bevy/blob/main/examples/asteroids.rs), for an example that uses noise to procedurally generate a tilemap on the CPU and a matching background in a wgsl shader.

## Bevy Version Support

The `main` branch targets the latest bevy release.

|bevy|noisy_bevy|
|----|----------|
|0.10| 0.3, main|
|0.9 | 0.2      |
|0.8 | 0.1      |

## License

MIT

The original simplex noise source is MIT-only, however all changes made by me or PRs to this repo are also available under Apache-2.0.

## Acknowledgments

The noise primitives are ports/copies of these

- <https://github.com/stegu/psrdnoise>
- <https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39>

## Contributions

PRs welcome!