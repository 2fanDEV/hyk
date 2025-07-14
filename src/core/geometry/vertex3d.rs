use glm::{Vector2, Vector3, Vector4};

#[derive(Copy, Clone, Debug)]
pub struct Vertex3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub uv: Vector2<f32>,
    pub normals: Vector3<f32>,
    pub colors: Vector4<f32>
}



