use std::fmt::Debug;


use ui::Mesh;

use super::{buffers::MeshBuffer, geometry::vertex3d::Vertex3D};

pub mod identifier;
pub mod objects;
pub mod ui;

pub trait Renderable : Debug {
    fn new() -> Self where Self: Sized;
    fn mesh(&self) -> &Mesh<Vertex3D>;
    fn update_buffers(&mut self);
    fn get_buffers(&mut self) -> MeshBuffer<Vertex3D>;
}
