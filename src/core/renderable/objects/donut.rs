use glm::{Vec3, Vector3};

use crate::core::{
    buffers::MeshBuffer, geometry::vertex3d::Vertex3D, renderable::{ui::Mesh, Renderable}
};

#[derive(Debug)]
pub struct Donut {
    position: Vector3<f32>,
}

impl Renderable for Donut {
    fn new() -> Self
    where
        Self: Sized,
    {
       Self {
           position: Vector3::new(0.0, 0.0, 0.0)
                 
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
