/// A mesh component describes a shape that can be rendered with a GPU.
#[derive(Clone, Debug)]
pub struct Mesh<V: crate::graphics::Vertex> {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
}

impl<V: crate::graphics::Vertex> Mesh<V> {
    /// Returns the number of vertices in the mesh.
    pub fn num_vertices(&self) -> u32 {
        self.vertices.len() as u32
    }

    /// Returns the number of indices in the mesh.
    pub fn num_indices(&self) -> u32 {
        self.indices.len() as u32
    }
}
