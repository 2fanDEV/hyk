pub mod identifier;

pub trait Renderable {
    pub fn meshes() -> Mesh<Vertex3D>;
}



