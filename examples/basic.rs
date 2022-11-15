//! Shows how to sample noise on the CPU.
//!
//! Generates a simple fbm island height map on the CPU and spawns tiles
//! with corresponding colors.

use bevy::{math::vec2, prelude::*, render::camera::ScalingMode};
use bevy_pancam::{PanCam, PanCamPlugin};
use noisy_bevy::fbm_simplex_2d;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.projection.scaling_mode = ScalingMode::FixedVertical(70.);
    commands.spawn((cam, PanCam::default()));

    const FREQUENCY_SCALE: f32 = 0.05;
    const AMPLITUDE_SCALE: f32 = 4.0;
    const RADIUS: f32 = 30.;
    const OCTAVES: usize = 3;
    const LACUNARITY: f32 = 2.;
    const GAIN: f32 = 0.5;

    let grid_half_size = RADIUS as i32 + 1;

    for x in -grid_half_size..=grid_half_size {
        for y in -grid_half_size..=grid_half_size {
            let p = vec2(x as f32, y as f32);

            // this is the whole point of the example
            let offset =
                fbm_simplex_2d(p * FREQUENCY_SCALE, OCTAVES, LACUNARITY, GAIN) * AMPLITUDE_SCALE;

            let height = RADIUS + offset - ((x * x + y * y) as f32).sqrt();

            // spawn a corresponding tile with a color thats more white the higher the height
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE * height * 0.03,
                    custom_size: Some(Vec2::splat(1.)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 100.)),
                ..default()
            });
        }
    }
}
