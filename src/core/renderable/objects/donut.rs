use std::{path::Path, sync::Arc};

use nalgebra::Vector3;

use crate::core::{
    buffers::{self, MeshBuffer},
    device::WGPUDevice,
    geometry::vertex3d::Vertex3D,
    renderable::{
        ui::{Mesh, Scissor},
        Renderable, RenderableSealed,
    },
    scene_manager::parser::MeshLoader,
};

#[derive(Debug)]
pub struct Donut {
    position: Vector3<f32>,
    meshes: Vec<Mesh<Vertex3D>>,
    mesh_buffers: Vec<MeshBuffer<Vertex3D>>,
}

impl RenderableSealed for Donut {}
impl Renderable for Donut {
    fn new(path: &Path, mesh_loader: Arc<MeshLoader>, label: Option<&str>, device: &WGPUDevice) -> Self
        where 
        Self: Sized,
    {
        let (meshes, mesh_buffers) = Self::init(path, mesh_loader, label, device);
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
