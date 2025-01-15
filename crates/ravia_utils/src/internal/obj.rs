use std::io::{BufReader, Read};

use ravia_engine::prelude::*;

use super::resource::read_resource;

/// Loads a mesh from a buffer containing an OBJ-formatted buffer.
///
/// This function expects an .obj buffer with vertex data, together with optional vertex colors,
/// normals, or texture coordinates. The mesh will be composed with appropriate data type.
pub fn load_mesh_from_obj<R: Read>(ctx: &EngineContext, read: R) -> crate::Result<Mesh> {
    let mut buf = BufReader::new(read);
    let (models, _) = tobj::load_obj_buf(
        &mut buf,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
        |path| {
            let path = path.to_str().expect("invalid path");
            let res = read_resource(path).map_err(|_| tobj::LoadError::OpenFileFailed)?;
            let mut buf = BufReader::new(res);
            tobj::load_mtl_buf(&mut buf)
        },
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
        for i in 0..num_vertices {
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
