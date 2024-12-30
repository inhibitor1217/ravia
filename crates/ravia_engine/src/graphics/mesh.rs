use wgpu::util::DeviceExt;

use crate::{ecs, engine::EngineContext};

/// A trait for vertex data.
///
/// The data type implementing this trait contains data for a single vertex, which should describe
/// the attributes and their formats.
pub trait Vertex: bytemuck::Pod + bytemuck::Zeroable {
    const ATTRIBUTE_FORMATS: &[wgpu::VertexFormat];
    const SIZE: u64 = std::mem::size_of::<Self>() as u64;
}

/// A 2D vertex with a custom data type.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Zeroable)]
pub struct Vertex2D<D>
where
    D: bytemuck::Pod + bytemuck::Zeroable,
{
    pub position: [f32; 2],
    pub data: D,
}

unsafe impl<D: bytemuck::Pod + bytemuck::Zeroable> bytemuck::Pod for Vertex2D<D> {}

/// A 2D vertex with a color.
pub type Vertex2DColor = Vertex2D<[f32; 3]>;

/// A 2D vertex with a texture coordinate.
pub type Vertex2DTexture = Vertex2D<[f32; 2]>;

impl Vertex for Vertex2DColor {
    const ATTRIBUTE_FORMATS: &[wgpu::VertexFormat] =
        &[wgpu::VertexFormat::Float32x2, wgpu::VertexFormat::Float32x3];
}

impl Vertex for Vertex2DTexture {
    const ATTRIBUTE_FORMATS: &[wgpu::VertexFormat] =
        &[wgpu::VertexFormat::Float32x2, wgpu::VertexFormat::Float32x2];
}

/// A mesh component describes a shape that can be rendered with a GPU.
#[derive(Debug)]
pub struct Mesh {
    pub(super) vertex_buffer: wgpu::Buffer,
    pub(super) index_buffer: wgpu::Buffer,

    num_vertices: u32,
    num_indices: u32,
}

assert_impl_all!(Mesh: ecs::storage::Component);

impl Mesh {
    /// Creates a new [`Mesh`] from vertex data.
    ///
    /// This is a convenience function that creates an indexed mesh with the default indices.
    pub fn new<V: Vertex>(ctx: &EngineContext, vertices: Vec<V>) -> Self {
        let indices = (0..vertices.len() as u32).collect();
        Self::new_indexed(ctx, vertices, indices)
    }

    /// Creates a new [`Mesh`] from vertex and index data.
    ///
    /// For now, we are allocating a new buffer for each mesh. This can be later optimized by allocating
    /// a large buffer for multiple meshes and tracking their offset.
    pub fn new_indexed<V: Vertex>(
        ctx: &EngineContext,
        vertices: Vec<V>,
        indices: Vec<u32>,
    ) -> Self {
        let vertex_buffer = ctx
            .gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = ctx
            .gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        Self {
            vertex_buffer,
            index_buffer,

            num_vertices: vertices.len() as u32,
            num_indices: indices.len() as u32,
        }
    }

    /// Returns the number of vertices in the mesh.
    pub fn num_vertices(&self) -> u32 {
        self.num_vertices
    }

    /// Returns the number of indices in the mesh.
    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }

    /// Returns the index range of the mesh.
    pub fn indices(&self) -> std::ops::Range<u32> {
        0..self.num_indices
    }

    /// Returns a slice of the vertex buffer to bind for a render pass.
    pub(super) fn vertex_slice(&self) -> wgpu::BufferSlice {
        self.vertex_buffer.slice(..)
    }

    /// Returns a slice of the index buffer to bind for a render pass.
    pub(super) fn index_slice(&self) -> wgpu::BufferSlice {
        self.index_buffer.slice(..)
    }
}
