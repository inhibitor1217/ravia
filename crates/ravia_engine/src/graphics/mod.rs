// implementation module
mod internal;
mod system;

pub use internal::{
    camera::Camera,
    gpu::Gpu,
    material::Material,
    mesh::{Mesh, Vertex, Vertex2D, Vertex2DColor, Vertex2DTexture, Vertex3D, Vertex3DTexture},
    shader::{Shader, ShaderConfig},
    texture::{Texture, TextureFilterMode},
    uniform::{Uniform, UniformType},
};
pub use system::system;
