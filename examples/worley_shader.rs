use bevy::{
    math::vec2,
    prelude::*,
    render::{camera::ScalingMode, render_resource::AsBindGroup},
    sprite::{Material2d, Material2dPlugin},
};
use noisy_bevy::NoisyShaderPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            NoisyShaderPlugin,
            Material2dPlugin::<BackgroundMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct BackgroundMaterial {
    #[uniform(0)]
    params: Vec2,
}

impl Material2d for BackgroundMaterial {
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "examples/worley_background.wgsl".into()
    }
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "examples/worley_background.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 230.0,
            },
            ..OrthographicProjection::default_2d()
        },
    ));

    let frequency_scale = 0.1;
    let amplitude_scale = 1.0;

    let material_handle = materials.add(BackgroundMaterial {
        params: vec2(frequency_scale, amplitude_scale),
    });
    let mesh_handle = meshes.add(Mesh::from(Rectangle::from_size(Vec2::new(1000.0, 1000.0))));

    commands.spawn((
        Transform::default(),
        Mesh2d(mesh_handle),
        MeshMaterial2d(material_handle),
    ));
}
