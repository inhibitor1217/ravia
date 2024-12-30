use core::fmt;

use crate::{ecs, engine::EngineContext};

use super::{
    shader::{Shader, ShaderConfig},
    texture::Texture,
};

/// A [`Material`] component describes how the shape should be rendered.
pub struct Material {
    pub shader: Shader,
    pub texture: Option<Texture>,
}

assert_impl_all!(Material: ecs::storage::Component);

impl Material {
    /// Creates a new [`Material`].
    pub fn new(ctx: &EngineContext, shader_config: &ShaderConfig) -> Self {
        Self {
            shader: Shader::new(ctx, shader_config),
            texture: None,
        }
    }
}

impl fmt::Debug for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Material")
    }
}
