use bevy::{
    asset::AssetServerSettings,
    math::{vec2, vec4},
    prelude::*,
    reflect::TypeUuid,
    render::{camera::ScalingMode, render_resource::AsBindGroup},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use noisy_bevy::{simplex_2d, ShaderNoisePlugin};

fn main() {
    App::new()
        .register_type::<AsteroidParams>()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShaderNoisePlugin)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(Material2dPlugin::<AsteroidBackgroundMaterial>::default())
        .add_plugin(WorldInspectorPlugin::default())
        .add_startup_system(setup)
        .add_system(expand_asteroids)
        .run();
}

fn setup(mut commands: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.projection.scaling_mode = ScalingMode::FixedVertical(50.);

    commands.spawn_bundle(cam).insert(PanCam::default());

    commands.spawn_bundle(AsteroidBundle::default());
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "1e449d2e-6901-4bff-95fa-d7407ad62b58"]
struct AsteroidBackgroundMaterial {
    #[uniform(0)]
    params: Vec4,
}

impl Material2d for AsteroidBackgroundMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "examples/asteroid_background.wgsl".into()
    }
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
struct AsteroidParams {
    frequency_scale: f32,
    amplitude_scale: f32,
    radius: f32,
}

impl Default for AsteroidParams {
    fn default() -> Self {
        Self {
            frequency_scale: 0.1,
            amplitude_scale: 2.8,
            radius: 14.0,
        }
    }
}

#[derive(Bundle)]
struct AsteroidBundle {
    name: Name,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
    params: AsteroidParams,
}

impl Default for AsteroidBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Asteroid"),
            transform: default(),
            global_transform: default(),
            visibility: default(),
            computed_visibility: default(),
            params: default(),
        }
    }
}

// turns compact model representation into something we can see on screen
fn expand_asteroids(
    changed_asteroids: Query<(Entity, &AsteroidParams), Changed<AsteroidParams>>,
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asteroid_materials: ResMut<Assets<AsteroidBackgroundMaterial>>,
) {
    // todo: bevy_asset_loader
    // let asteroid_sdf = asset_server.load("asteroid_sdf.wgsl");

    for (asteroid_entity, params) in changed_asteroids.iter() {
        let max_half_size = params.radius as i32 + 1;

        commands.entity(asteroid_entity).despawn_descendants();
        commands.entity(asteroid_entity).with_children(|asteroid| {
            for x in -max_half_size..=max_half_size {
                for y in -max_half_size..=max_half_size {
                    let p = vec2(x as f32, y as f32);
                    let o = simplex_2d(p * params.frequency_scale) * params.amplitude_scale;
                    if ((x * x + y * y) as f32) < (params.radius + o).powi(2) {
                        asteroid.spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: Color::GRAY,
                                custom_size: Some(Vec2::splat(1.)),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(
                                x as f32, y as f32, 100.,
                            )),
                            ..default()
                        });
                    }
                }
            }

            // we are making a new material each time we make an asteroid
            // this doesn't really scale well, but works fine for an example
            let material_handle = asteroid_materials.add(AsteroidBackgroundMaterial {
                params: vec4(
                    params.frequency_scale,
                    params.amplitude_scale,
                    params.radius,
                    0.,
                ),
            });

            let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(100.0, 100.0))));

            asteroid.spawn_bundle(MaterialMesh2dBundle {
                mesh: quad_handle.into(),
                material: material_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.5),
                    ..default()
                },
                ..default()
            });
        });
    }
}
