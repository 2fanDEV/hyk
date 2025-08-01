use bytemuck::{Pod, Zeroable};
use nalgebra::{Vector2, Vector3, Vector4};
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, PushConstantRange, ShaderStages, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode
};

use super::{BindingGroupLayoutInformation, PushConstants, VertexStateInformation};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex3D {
    pub position: Vector3<f32>,
    pub uv: Vector2<f32>,
    pub normals: Vector3<f32>,
    pub colors: Vector4<f32>,
}

impl VertexStateInformation for Vertex3D {
    fn vertex_state() -> Vec<wgpu::VertexBufferLayout<'static>> {
        vec![VertexBufferLayout {
            array_stride: 48,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 1,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 2,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 3,
                },
            ],
            step_mode: VertexStepMode::Vertex,
        }]
    }
}

impl BindingGroupLayoutInformation for Vertex3D {
    fn binding_group_layouts(device: &wgpu::Device) -> Vec<wgpu::BindGroupLayout> {
        vec![device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Scene Data Frag"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: true, min_binding_size: None },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        }),
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Scene Data Frag"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: true, min_binding_size: None },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
        ]
    }
}

impl PushConstants for Vertex3D {}
