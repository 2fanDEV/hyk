use std::{path::Path, sync::Arc};

use nalgebra::Vector3;

use crate::core::{
    buffers::MeshBuffer, device::WGPUDevice, geometry::vertex3d::Vertex3D, renderable::{
        ui::Mesh,
        Renderable, RenderableSealed,
    }, resource_manager::TextureHandle, scene_manager::parser::MeshLoader
};

#[derive(Debug)]
pub struct Donut {
    position: Vector3<f32>,
    meshes: Vec<Mesh<Vertex3D>>,
    mesh_buffers: Vec<MeshBuffer<Vertex3D>>,
    texture_handle: TextureHandle
}

impl RenderableSealed for Donut {
    fn get_meshes(&self) -> &[Mesh<Vertex3D>] {
        &self.meshes
    }
}
impl Renderable for Donut {
    fn new(mesh_loader: Arc<MeshLoader>, label: Option<&str>, device: Arc<WGPUDevice>) -> Self
        where 
        Self: Sized,
    {
        let (meshes, mesh_buffers) = Self::init(Path::new("assets/donut.glb"), mesh_loader, label, &device);
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            meshes,
            mesh_buffers,
            texture_handle: TextureHandle::VERTEX_3D
        }
    }

    fn update_buffers(&mut self) {
        todo!()
    }

    fn get_buffers(&mut self) -> MeshBuffer<Vertex3D> {
        todo!()
    }

    fn get_texture_handle(&self) -> &TextureHandle {
        &self.texture_handle
    }
}
