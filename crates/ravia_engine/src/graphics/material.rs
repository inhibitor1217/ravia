use crate::{ecs, engine::EngineContext};

use super::{Shader, ShaderConfig};

/// A [`Material`] component describes how the shape should be rendered.
#[derive(Debug)]
pub struct Material {
    pub(super) shader: Shader,
}

assert_impl_all!(Material: ecs::storage::Component);

impl Material {
    /// Creates a new [`Material`].
    pub fn new(ctx: &EngineContext, shader_config: &ShaderConfig) -> Self {
        Self {
            shader: Shader::new(ctx, shader_config),
        }
    }
}
