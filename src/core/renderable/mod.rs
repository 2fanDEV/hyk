use std::{fmt::Debug, path::Path, sync::Arc};


use ui::Mesh;
use wgpu::BufferUsages;

use super::{buffers::{self, MeshBuffer}, device::WGPUDevice, geometry::vertex3d::Vertex3D, scene_manager::parser::MeshLoader};

pub mod identifier;
pub mod objects;
pub mod ui;

trait RenderableSealed: Debug {
    fn init(path: &Path, mesh_loader: Arc<MeshLoader>, label: Option<&str>, device: &WGPUDevice) -> (Vec<Mesh<Vertex3D>>, Vec<MeshBuffer<Vertex3D>>)
    where Self: Sized
    {
        let label_or_default = match label {
            Some(label) => label,
            None => "DEFAULT",
        };
        let meshes = mesh_loader.load_mesh(path);
        let mut mesh_buffers = vec![];
        for mesh in &meshes {
            let vertex_buffer = buffers::ElementBuffer::new_mapped(
                device,
                Some(&(label_or_default.to_string() + "_VERTICES")),
                BufferUsages::VERTEX,
                Some(mesh.scissor),
                buffers::ElementType::VECTOR(mesh.vertices.clone()),
            )
            .unwrap();

            let index_buffer = buffers::ElementBuffer::new_mapped(
                device,
                Some(&(label_or_default.to_string() + "_INDICES")),
                BufferUsages::INDEX,
                Some(mesh.scissor),
                buffers::ElementType::VECTOR(mesh.indices.clone()),
            )
            .unwrap();
            mesh_buffers.push(MeshBuffer::<Vertex3D>::new(vertex_buffer, index_buffer));   
        };
        (meshes, mesh_buffers)
    }
}


#[allow(private_bounds)]
pub trait Renderable : RenderableSealed {
    fn new(path: &Path, mesh_loader: Arc<MeshLoader>, label: Option<&str>,  device: &WGPUDevice) -> Self where Self: Sized;
    fn mesh(&self) -> &Mesh<Vertex3D>;
    fn update_buffers(&mut self);
    fn get_buffers(&mut self) -> MeshBuffer<Vertex3D>;
}
