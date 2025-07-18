use glm::{Vector2, Vector3, Vector4};

use super::{BindingGroupLayoutInformation, PushConstants, VertexStateInformation};

#[derive(Copy, Clone, Debug)]
pub struct Vertex3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub uv: Vector2<f32>,
    pub normals: Vector3<f32>,
    pub colors: Vector4<f32>
}

impl VertexStateInformation for Vertex3D {
    fn vertex_state() -> Vec<wgpu::VertexBufferLayout<'static>> {
        todo!()
    }
}

impl BindingGroupLayoutInformation for Vertex3D {
    fn binding_group_layouts(device: &wgpu::Device) -> Vec<wgpu::BindGroupLayout> {
        todo!()
    }
}

impl PushConstants for Vertex3D {
    fn push_constant_ranges<T>() -> wgpu::PushConstantRange {
        todo!()
    }
}



