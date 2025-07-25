use std::num::{NonZero, NonZeroU32};

use bytemuck::Pod;
use egui::epaint::Vertex;
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Device, PushConstantRange, SamplerBindingType, ShaderStages, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode
};

pub mod vertex3d;

pub trait VertexStateInformation {
    fn vertex_state() -> Vec<VertexBufferLayout<'static>>;
}

pub trait BindingGroupLayoutInformation {
    fn binding_group_layouts(device: &Device) -> Vec<BindGroupLayout>;
}

pub trait PushConstants {
    fn push_constant_ranges<T>() -> PushConstantRange;
}

impl VertexStateInformation for Vertex {
    fn vertex_state() -> Vec<VertexBufferLayout<'static>> {
        vec![VertexBufferLayout {
            array_stride: 20,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 8,
                    shader_location: 1,
                },
                VertexAttribute {
                    format: VertexFormat::Unorm8x4,
                    offset: 16,
                    shader_location: 2,
                },
            ],
        }]
    }
}

impl PushConstants for Vertex {
    fn push_constant_ranges<T>() -> PushConstantRange {
        PushConstantRange { stages: ShaderStages::VERTEX, range: 0..size_of::<T>() as u32 }
    }
}

impl BindingGroupLayoutInformation for Vertex {
    fn binding_group_layouts(device: &Device) -> Vec<BindGroupLayout> {
        vec![device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Fragment 2D"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }, BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                count: None, 
                ty: BindingType::Sampler(SamplerBindingType::Filtering)
            }],
        })]
    }
}
