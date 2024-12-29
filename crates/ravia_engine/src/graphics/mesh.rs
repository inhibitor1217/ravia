use std::sync::Arc;

use wgpu::util::DeviceExt;

use super::Gpu;

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

impl Vertex for Vertex2DColor {
    const ATTRIBUTE_FORMATS: &[wgpu::VertexFormat] =
        &[wgpu::VertexFormat::Float32x2, wgpu::VertexFormat::Float32x3];
}

/// A mesh component describes a shape that can be rendered with a GPU.
#[derive(Clone, Debug)]
pub struct Mesh<V: Vertex> {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,

    pub(super) buffers: Option<MeshBuffers>,
}

impl<V: Vertex> Mesh<V> {
    /// Creates a new [`Mesh`].
    pub fn new(vertices: Vec<V>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            buffers: None,
        }
    }

    /// Returns the number of vertices in the mesh.
    pub fn num_vertices(&self) -> u32 {
        self.vertices.len() as u32
    }

    /// Returns the number of indices in the mesh.
    pub fn num_indices(&self) -> u32 {
        self.indices.len() as u32
    }

    /// Allocates the buffers for the mesh if they are not already allocated.
    pub(super) fn maybe_allocate(&mut self, gpu: &Gpu) -> &MeshBuffers {
        if self.buffers.is_none() {
            let vertex_buffer = gpu
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&self.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

            let index_buffer = gpu
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&self.indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

            self.buffers = Some(MeshBuffers {
                vertex_buffer: Arc::new(vertex_buffer),
                index_buffer: Arc::new(index_buffer),
            });
        }

        self.buffers.as_ref().unwrap()
    }
}

/// Handles for the underlying buffers allocated in the GPU for the mesh.
///
/// For now, we just use a simple strategy to allocate new buffers for each mesh.
/// This can be optimized later.
#[derive(Clone, Debug)]
struct MeshBuffers {
    pub vertex_buffer: Arc<wgpu::Buffer>,
    pub index_buffer: Arc<wgpu::Buffer>,
}
