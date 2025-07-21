use std::{fmt::Debug, path::Path, sync::Arc};


use ui::Mesh;

use super::{buffers::MeshBuffer, device::WGPUDevice, geometry::vertex3d::Vertex3D, scene_manager::parser::MeshLoader};

pub mod identifier;
pub mod objects;
pub mod ui;

pub trait Renderable : Debug {
    fn new(path: &Path, mesh_loader: Arc<MeshLoader>, device: &WGPUDevice) -> Self where Self: Sized;
    fn mesh(&self) -> &Mesh<Vertex3D>;
    fn update_buffers(&mut self);
    fn get_buffers(&mut self) -> MeshBuffer<Vertex3D>;
}
