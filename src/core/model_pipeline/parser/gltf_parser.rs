use std::path::Path;

use anyhow::Result;

use crate::core::{geometry::vertex3d::Vertex3D, ui::Meshes};

fn parse_gltf(path: &Path) -> Result<Vec<Meshes>> {
    let gltf_model_info = gltf::Gltf::open(path).unwrap();
    let meshes = gltf_model_info.meshes();
    let blob = &gltf_model_info.blob;
    let mut meshes_res: Vec<Meshes> = vec![];
    let vertices: Vec<Vertex3D> = vec![];
    let mut indices: Vec<u32> = vec![];
    for mesh in meshes {
        vertices.clear();
        indices.clear();
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| blob.as_deref());
            let indices = reader
                .read_indices()
                .unwrap()
                .into_u32()
                .collect::<Vec<_>>();
            let normals = reader.read_normals().unwrap().collect::<Vec<_>>();
            let vertex_positions = reader.read_positions().unwrap().collect::<Vec<_>>();
            for (i, position) in vertex_positions.iter().enumerate() {
                let normal = normals.get(i).unwrap_or(&[0.0, 0.0, 0.0]);
                vertices.push(Vertex3D {
                    x: position[0],
                    y: position[1],
                    z: position[2],
                });

                meshes_res.push(Meshes {
                    vertices,
                    indices,
                    texture_id: Default::default(),
                    scissor: Scissor {
                        width: 0,
                        height: 0,
                        x: 0,
                        y: 0,
                    },
                });
            }
        }
    }
    Ok(meshes)
}
