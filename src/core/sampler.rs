use anyhow::Result;
use wgpu::{Device, Sampler, SamplerBorderColor, SamplerDescriptor};

pub fn create_egui_sampler(device: &Device) -> Result<Sampler> {
    Ok(device.create_sampler(&SamplerDescriptor {
        label: Some("Egui Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        border_color: Some(SamplerBorderColor::OpaqueBlack),
        ..Default::default()
    }))
}
