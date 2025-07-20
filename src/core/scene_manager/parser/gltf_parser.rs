use std::path::Path;

use anyhow::{anyhow, Result};
use glm::{Vector2, Vector3, Vector4};
use log::debug;

use crate::core::{
    geometry::vertex3d::Vertex3D, renderable::ui::{Mesh, Scissor},
};

use super::Loader;

pub struct GltfLoader {}

impl GltfLoader {
    pub fn new() -> Self {
        Self {}
    }
}

impl Loader for GltfLoader {
    fn load(&self, path: &Path) -> Result<Vec<Mesh<Vertex3D>>> {
        let gltf_model_info = gltf::Gltf::open(path).unwrap();
        let meshes = gltf_model_info.meshes();
        let blob = &gltf_model_info.blob;
        let mut meshes_res: Vec<Mesh<Vertex3D>> = vec![];
        let mut vertices: Vec<Vertex3D> = vec![];
        for (index, mesh) in meshes.enumerate() {
            vertices.clear();
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|_buffer| blob.as_deref());
                let indices = reader
                    .read_indices()
                    .unwrap()
                    .into_u32()
                    .collect::<Vec<_>>();
                let normals = reader.read_normals().unwrap().collect::<Vec<_>>();
                let colors = match reader.read_colors(index as u32) {
                    Some(colors) => colors.into_rgba_f32().collect::<Vec<[f32; 4]>>(),
                    None => normals
                        .iter()
                        .map(|normal| [normal[0], normal[1], normal[2], 1.0])
                        .collect::<Vec<_>>(),
                };
                let uv = reader
                    .read_tex_coords(0)
                    .ok_or(anyhow!("There are no UVs"))?
                    .into_f32()
                    .collect::<Vec<_>>();
                let color_override = false;
                let white_color = [1.0, 1.0, 1.0, 1.0];

                let vertex_positions = reader.read_positions().unwrap().collect::<Vec<_>>();
                for (i, position) in vertex_positions.iter().enumerate() {
                    let normal = normals.get(i).unwrap_or(&[0.0, 0.0, 0.0]);
                    let color_arr: &[f32; 4] = if color_override {
                        &white_color
                    } else {
                        &colors[i]
                    };
                    let uvs = uv[i];
                    vertices.push(Vertex3D {
                        x: position[0],
                        y: position[1],
                        z: position[2],
                        uv: Vector2::new(uvs[0], uvs[1]),
                        normals: Vector3::new(normal[0], normal[1], normal[2]),
                        colors: {
                            Vector4::<f32>::new(
                                color_arr[0],
                                color_arr[1],
                                color_arr[2],
                                color_arr[3],
                            )
                        },
                    });

                    meshes_res.push(Mesh {
                        vertices: vertices.clone(),
                        indices: indices.clone(),
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
        debug!("{meshes_res:?}");
        Ok(meshes_res)
    }
}
#[cfg(test)]
mod tests {}
