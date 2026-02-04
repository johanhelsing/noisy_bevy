use bevy::{
    camera::ScalingMode,
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite_render::{Material2d, Material2dPlugin},
};
use noisy_bevy::NoisyShaderPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            NoisyShaderPlugin,
            Material2dPlugin::<NoiseMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct NoiseMaterial {
    #[uniform(0)]
    num_warps: i32,
}

impl Material2d for NoiseMaterial {
    fn vertex_shader() -> bevy::shader::ShaderRef {
        "examples/domain_warping.wgsl".into()
    }
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "examples/domain_warping.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NoiseMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 230.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    let material_handle = materials.add(NoiseMaterial { num_warps: 4 });
    let mesh_handle = meshes.add(Mesh::from(Rectangle::from_size(Vec2::new(500.0, 500.0))));

    commands.spawn((
        Transform::default(),
        Mesh2d(mesh_handle),
        MeshMaterial2d(material_handle),
    ));
}
