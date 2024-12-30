/// A trait for uniform variables.
pub trait Uniform {
    /// Specifies the bind group of the uniform variable.
    fn bind_group(&self) -> &wgpu::BindGroup;

    /// Specifies the uniform type, which marks the bind group layout to use.
    fn uniform_type(&self) -> UniformType;
}

/// Specifies the type of the uniform variable, whose bindings are
/// supported from the engine by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UniformType {
    /// Binds a 2D [`super::texture::Texture`] type as a uniform.
    Texture2D,
    /// Binds a [`super::camera::Camera`] type as a uniform.
    Camera,
}
