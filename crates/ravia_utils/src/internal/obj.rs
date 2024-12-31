use std::{fmt::Debug, path::Path};

use ravia_engine::prelude::*;

/// Loads a mesh from an OBJ file.
///
/// This function expects an .obj file with vertex data, together with optional vertex colors,
/// normals, or texture coordinates. The mesh will be composed with appropriate data type.
pub fn load_mesh_from_obj<P: AsRef<Path> + Debug>(
    ctx: &EngineContext,
    path: P,
) -> crate::Result<Mesh> {
    let (models, _) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    )?;

    if models.is_empty() {
        return Err(anyhow::anyhow!("No models found in the OBJ file"));
    }

    let model = models.first().unwrap();

    let mesh = if model.mesh.vertex_color.is_empty() {
        let mut vertices = vec![];
        for i in 0..model.mesh.positions.len() {
            vertices.push(Vertex3DStandard {
                position: Vec3::from_slice(&model.mesh.positions[3 * i..3 * i + 3]),
                data: VertexStandardData {
                    normal: Vec3::from_slice(&model.mesh.normals[3 * i..3 * i + 3]),
                    uv: Vec2::from_slice(&model.mesh.texcoords[2 * i..2 * i + 2]),
                },
            });
        }
        Mesh::new_indexed(ctx, &vertices, &model.mesh.indices)
    } else {
        let mut vertices = vec![];
        for i in 0..model.mesh.positions.len() {
            vertices.push(Vertex3DStandardColored {
                position: Vec3::from_slice(&model.mesh.positions[3 * i..3 * i + 3]),
                data: VertexStandardColoredData {
                    normal: Vec3::from_slice(&model.mesh.normals[3 * i..3 * i + 3]),
                    uv: Vec2::from_slice(&model.mesh.texcoords[2 * i..2 * i + 2]),
                    color: Vec3::from_slice(&model.mesh.vertex_color[3 * i..3 * i + 3]),
                },
            });
        }
        Mesh::new_indexed(ctx, &vertices, &model.mesh.indices)
    };

    Ok(mesh)
}
