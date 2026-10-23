use bevy::{
    app::{App, Plugin},
    asset::{Handle, load_internal_asset, uuid_handle},
    shader::Shader,
};

/// Adds noise library as a wgsl import
///
/// General functionality can be imported through:
///
/// ```wesl
/// import noisy_bevy::simplex_noise_2d;
/// ```
pub struct NoisyShaderPlugin;

impl Plugin for NoisyShaderPlugin {
    fn build(&self, app: &mut App) {
        // not using embedded_asset, since it would make the import path noisy_bevy::<file>
        load_internal_asset!(
            app,
            NOISY_SHADER_HANDLE,
            "../assets/noisy_bevy.wesl",
            |source, _path| Shader::from_wesl(source, "embedded://noisy_bevy.wesl")
        );
    }
}

const NOISY_SHADER_HANDLE: Handle<Shader> = uuid_handle!("9e85d206-7851-41d9-a04f-c4879ddd7143");
