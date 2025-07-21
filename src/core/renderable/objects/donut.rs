use std::{path::Path, sync::Arc};

use nalgebra::Vector3;
use wgpu::BufferUsages;

use crate::core::{
    buffers::{self, MeshBuffer},
    device::WGPUDevice,
    geometry::vertex3d::Vertex3D,
    renderable::{ui::{Mesh, Scissor}, Renderable},
    scene_manager::parser::MeshLoader,
};

#[derive(Debug)]
pub struct Donut {
    position: Vector3<f32>,
    meshes: Vec<Mesh<Vertex3D>>,
    mesh_buffers: Vec<MeshBuffer<Vertex3D>>,
}

impl Renderable for Donut {
    fn new(path: &Path, mesh_loader: Arc<MeshLoader>, device: &WGPUDevice) -> Self
    where
        Self: Sized,
    {
        let meshes = mesh_loader.load_mesh(path);
        let mesh_buffers = vec![];
        for mesh in &meshes {
            buffers::ElementBuffer::new_mapped(
                device,
                Some("DONUT_VERTICES"),
                BufferUsages::VERTEX,
                Some(mesh.scissor),
                buffers::ElementType::VECTOR(mesh.vertices.clone()),
            );
        }
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            meshes,
            mesh_buffers,
        }
    }

    fn mesh(&self) -> &Mesh<Vertex3D> {
        todo!()
    }

    fn update_buffers(&mut self) {
        todo!()
    }

    fn get_buffers(&mut self) -> MeshBuffer<Vertex3D> {
        todo!()
    }
}
