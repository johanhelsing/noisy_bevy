use bevy::{math::vec2, prelude::*, render::camera::ScalingMode};
use noisy_bevy::fbm_simplex_2d_warp_seeded;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((DefaultPlugins,))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Msaa::Off,
        OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 230.0,
            },
            ..OrthographicProjection::default_2d()
        },
    ));

    let grid_half_size = 100;

    let frequency_scale = 0.005;
    let octaves = 10;
    let lacunarity = 2.9;
    let gain = 0.4;
    let seed = 324.0;
    let num_warps = 3;
    let scale = vec2(0.4, 0.4);
    let faloff = 0.1;

    for x in -grid_half_size..grid_half_size {
        for y in -grid_half_size..grid_half_size {
            let position = vec2(x as f32, y as f32);

            let result = fbm_simplex_2d_warp_seeded(
                position * frequency_scale,
                octaves,
                lacunarity,
                gain,
                seed,
                num_warps,
                scale,
                faloff,
            );
            let luminance = result.noise_value;

            commands.spawn((
                Sprite {
                    color: Color::WHITE.with_luminance(luminance),
                    custom_size: Some(Vec2::splat(1.0)),
                    ..default()
                },
                Transform::from_xyz(x as f32, y as f32, 0.0),
            ));
        }
    }
}
