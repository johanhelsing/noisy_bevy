# noisy_bevy

<!-- [![crates.io](https://img.shields.io/crates/v/noisy_bevy.svg)](https://crates.io/crates/noisy_bevy) -->
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
<!-- [![crates.io](https://img.shields.io/crates/d/noisy_bevy.svg)](https://crates.io/crates/noisy_bevy) -->
<!-- [![docs.rs](https://img.shields.io/docsrs/noisy_bevy)](https://docs.rs/noisy_bevy) -->

Simple stupid noise primitives for glam types (`Vec2`, `Vec3`) and wgsl.

Main motivations are:

- ergonomic usage with Bevy
- same results on rust and wgsl (not bit-level perfect, though)

Implemented:

- `simplex_2d`
- `simplex_3d` (not tested)

todo:

- seeded versions (extra permute step)
- `fbm_2d`
- `fbm_3d`
- maybe others, I want to keep the scope and complexity down, though

## Usage

Rust: Zero initialization, just call the noise functions:

```rust
use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d;

let p = Vec2::new(12.3, 45.6);
let value = simplex_noise_2d(p);
```

wgsl: Just add the plugin to the app:

```rust ignore
App::new()
    .add_plugin(ShaderNoisePlugin)
```

And use it in your shaders, with the same API as on the CPU-side:

```wgsl
#import noisy_bevy::prelude

// ...

let p = vec2(12.3, 45.6);
let value = simplex_noise_2d(p);
```

See the [`asteroids example`](./examples/asteroids.rs), for an example that uses noise to procedurally generate a tilemap on the CPU and a matching background in a wgsl shader.

## Bevy Version Support

The `main` branch targets the latest bevy release.

## License

MIT

The original noise source is MIT-only, however all changes made by me or PRs to this repo are also available under Apache-2.0.

## Acknowledgments

The noise primitives are ports/copies of these

- https://github.com/stegu/psrdnoise
- https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39

## Contributions

PRs welcome!