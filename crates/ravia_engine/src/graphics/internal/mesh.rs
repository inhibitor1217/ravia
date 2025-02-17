use std::io::BufReader;

use wgpu::util::DeviceExt;

use crate::{ecs, engine::EngineContext, math};

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
    pub position: math::Vec2,
    pub data: D,
}

unsafe impl<D: bytemuck::Pod + bytemuck::Zeroable> bytemuck::Pod for Vertex2D<D> {}

/// A 2D vertex with a color.
pub type Vertex2DColor = Vertex2D<math::Vec3>;

/// A 2D vertex with a texture coordinate.
pub type Vertex2DTexture = Vertex2D<math::Vec2>;

impl Vertex for Vertex2DColor {
    const ATTRIBUTE_FORMATS: &[wgpu::VertexFormat] =
        &[wgpu::VertexFormat::Float32x2, wgpu::VertexFormat::Float32x3];
}

impl Vertex for Vertex2DTexture {
    const ATTRIBUTE_FORMATS: &[wgpu::VertexFormat] =
        &[wgpu::VertexFormat::Float32x2, wgpu::VertexFormat::Float32x2];
}

// A 3D vertex with a custom data type.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Zeroable)]
pub struct Vertex3D<D>
where
    D: bytemuck::Pod + bytemuck::Zeroable,
{
    pub position: math::Vec3,
    pub data: D,
}

unsafe impl<D: bytemuck::Pod + bytemuck::Zeroable> bytemuck::Pod for Vertex3D<D> {}

/// A 3D vertex with a texture coordinate.
pub type Vertex3DTexture = Vertex3D<math::Vec2>;

impl Vertex for Vertex3DTexture {
    const ATTRIBUTE_FORMATS: &[wgpu::VertexFormat] =
        &[wgpu::VertexFormat::Float32x3, wgpu::VertexFormat::Float32x2];
}

/// A standard vertex with a normal and a texture coordinate.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Zeroable)]
pub struct VertexStandardData {
    pub uv: math::Vec2,
    pub normal: math::Vec3,
}

unsafe impl bytemuck::Pod for VertexStandardData {}

/// A 3D vertex with a normal and a texture coordinate.
pub type Vertex3DStandard = Vertex3D<VertexStandardData>;

impl Vertex for Vertex3DStandard {
    const ATTRIBUTE_FORMATS: &[wgpu::VertexFormat] = &[
        wgpu::VertexFormat::Float32x3,
        wgpu::VertexFormat::Float32x2,
        wgpu::VertexFormat::Float32x3,
    ];
}

/// A standard vertex with a normal, a texture coordinate, and a color.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Zeroable)]
pub struct VertexStandardColoredData {
    pub uv: math::Vec2,
    pub normal: math::Vec3,
    pub color: math::Vec3,
}

unsafe impl bytemuck::Pod for VertexStandardColoredData {}

/// A 3D vertex with a normal, a texture coordinate, and a color.
pub type Vertex3DStandardColored = Vertex3D<VertexStandardColoredData>;

impl Vertex for Vertex3DStandardColored {
    const ATTRIBUTE_FORMATS: &[wgpu::VertexFormat] = &[
        wgpu::VertexFormat::Float32x3,
        wgpu::VertexFormat::Float32x2,
        wgpu::VertexFormat::Float32x3,
        wgpu::VertexFormat::Float32x3,
    ];
}

/// A [`Mesh`] component describes a shape that can be rendered with a GPU.
#[derive(Debug)]
pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_vertices: u32,
    num_indices: u32,
}

assert_impl_all!(Mesh: ecs::storage::Component);

impl Mesh {
    /// Creates a new [`Mesh`] from vertex data.
    ///
    /// This is a convenience function that creates an indexed mesh with the default indices.
    pub fn new<V: Vertex>(ctx: &EngineContext, vertices: &[V]) -> Self {
        let indices = (0..vertices.len() as u32).collect::<Vec<_>>();
        Self::new_indexed(ctx, vertices, &indices)
    }

    /// Creates a new [`Mesh`] from vertex and index data.
    ///
    /// For now, we are allocating a new buffer for each mesh. This can be later optimized by allocating
    /// a large buffer for multiple meshes and tracking their offset.
    pub fn new_indexed<V: Vertex>(ctx: &EngineContext, vertices: &[V], indices: &[u32]) -> Self {
        let vertex_buffer = ctx
            .gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = ctx
            .gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
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

/// Loads a mesh from a buffer containing an OBJ-formatted buffer.
///
/// This function expects an .obj buffer with vertex data, together with optional vertex colors,
/// normals, or texture coordinates. The mesh will be composed with appropriate data type.
pub fn load_mesh_from_obj(ctx: &EngineContext, data: &[u8]) -> Result<Mesh, anyhow::Error> {
    let mut buf = BufReader::new(data);
    let (models, _) = tobj::load_obj_buf(
        &mut buf,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
        // we do not allow loading materials for now.
        |_| Err(tobj::LoadError::GenericFailure),
    )?;

    if models.is_empty() {
        return Err(anyhow::anyhow!("No models found in the OBJ file"));
    }

    let model = models.first().unwrap();
    let num_vertices = model.mesh.positions.len() / 3;

    let mesh = if model.mesh.vertex_color.is_empty() {
        let mut vertices = vec![];
        for i in 0..num_vertices {
            vertices.push(Vertex3DStandard {
                position: math::Vec3::from_slice(&model.mesh.positions[3 * i..3 * i + 3]),
                data: VertexStandardData {
                    normal: math::Vec3::from_slice(&model.mesh.normals[3 * i..3 * i + 3]),
                    uv: math::Vec2::from_slice(&model.mesh.texcoords[2 * i..2 * i + 2]),
                },
            });
        }
        Mesh::new_indexed(ctx, &vertices, &model.mesh.indices)
    } else {
        let mut vertices = vec![];
        for i in 0..num_vertices {
            vertices.push(Vertex3DStandardColored {
                position: math::Vec3::from_slice(&model.mesh.positions[3 * i..3 * i + 3]),
                data: VertexStandardColoredData {
                    normal: math::Vec3::from_slice(&model.mesh.normals[3 * i..3 * i + 3]),
                    uv: math::Vec2::from_slice(&model.mesh.texcoords[2 * i..2 * i + 2]),
                    color: math::Vec3::from_slice(&model.mesh.vertex_color[3 * i..3 * i + 3]),
                },
            });
        }
        Mesh::new_indexed(ctx, &vertices, &model.mesh.indices)
    };

    Ok(mesh)
}
