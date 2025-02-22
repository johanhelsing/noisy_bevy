use bevy::{math::vec2, prelude::*, render::camera::ScalingMode};
use noisy_bevy::worley_2d;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
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
    let frequency_scale = 0.05;
    let jitter = 1.0;

    for x in -grid_half_size..grid_half_size {
        for y in -grid_half_size..grid_half_size {
            let position = vec2(x as f32, y as f32);
            let value = worley_2d(position * frequency_scale, jitter);
            let luminance = value.x;

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
