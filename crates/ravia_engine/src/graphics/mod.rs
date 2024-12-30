// implementation module
mod internal;
mod system;

pub use internal::{
    gpu::Gpu,
    material::Material,
    mesh::{Mesh, Vertex, Vertex2D, Vertex2DColor, Vertex2DTexture, Vertex3D},
    shader::{Shader, ShaderConfig},
    texture::{Texture, TextureFilterMode},
    uniform::{Uniform, UniformType},
};
pub use system::system;
