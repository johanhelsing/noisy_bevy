use bevy::{
    app::{ Plugin, App },
    asset::{ Handle, load_internal_asset, uuid_handle },
    shader::{ Shader },
};

/// Adds noise library as a wgsl import
///
/// General functionality can be imported through:
///
/// ```wgsl
/// #import noisy_bevy
/// ```
pub struct NoisyShaderPlugin;

impl Plugin for NoisyShaderPlugin {
    fn build(&self, app: &mut App) {
        // workaround: embedded_asset is broken in bevy 0.12.0
        load_internal_asset!(
            app,
            NOISY_SHADER_HANDLE,
            "../assets/noisy_bevy.wgsl",
            Shader::from_wgsl
        );
    }
}

const NOISY_SHADER_HANDLE: Handle<Shader> = uuid_handle!("9e85d206-7851-41d9-a04f-c4879ddd7143");

