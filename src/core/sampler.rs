use anyhow::Result;
use wgpu::{ naga::back::msl::sampler::BorderColor, CompareFunction, Device, Sampler, SamplerBorderColor, SamplerDescriptor};

pub fn create_egui_sampler(device: &Device) -> Result<Sampler> {
    Ok(device.create_sampler(&SamplerDescriptor {
        label: Some("Egui Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: 0.0,
        lod_max_clamp: 0.0,
        compare: None,
        anisotropy_clamp: 1,
        border_color: Some(SamplerBorderColor::OpaqueBlack)
    }))
}
