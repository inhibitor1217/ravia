/// A trait for uniform variables.
pub trait Uniform {
    const TYPE: UniformType;

    /// Specifies the bind group of the uniform variable.
    fn bind_group(&self) -> &wgpu::BindGroup;
}

/// Specifies the type of the uniform variable, whose bindings are
/// supported from the engine by default.
#[derive(Debug, Clone, Copy)]
pub enum UniformType {
    /// Binds a [`super::Texture2D`] type as a uniform.
    Texture2D,
}
