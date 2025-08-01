use std::{collections::HashMap, sync::Arc};

use strum_macros::AsRefStr;
use wgpu::{
    Extent3d, TextureDescriptor, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor, TextureViewDimension,
};

use super::{device::WGPUDevice, renderable::Renderable};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, AsRefStr)]
pub enum TextureHandle {
    #[strum(serialize = "2D")]
    VERTEX_2D,
    VERTEX_3D,
    CUSTOM_HANDLE(&'static str),
}

#[derive(Clone)]
pub struct TextureDetail {
    count: u32,
    texture_view: TextureView,
}

pub struct ResourceManager {
    texture_views: HashMap<TextureHandle, TextureDetail>,
    device: Arc<WGPUDevice>,
}

impl ResourceManager {
    pub fn new(device: Arc<WGPUDevice>) -> Self {
        Self {
            texture_views: HashMap::new(),
            device,
        }
    }

    //TODO: FIX Clones and change return a reference of a TextureDetail
    pub fn get_or_create(&mut self, object: &dyn Renderable) -> TextureDetail {
        let texture_detail = self.texture_views.get(object.get_texture_handle());
        return match texture_detail {
            Some(texture_detail) => texture_detail.clone(),
            None => {
                let texture = self.device.create_texture(&TextureDescriptor {
                    label: Some(&(object.get_texture_handle().as_ref().to_string() + "TEXTURE: ")),
                    size: Extent3d::default(), //TODO,
                    mip_level_count: 0,
                    sample_count: 0,
                    dimension: wgpu::TextureDimension::D3,
                    usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                    format: wgpu::TextureFormat::R32Float,
                    view_formats: &[TextureFormat::R32Float],
                });
                let texture_view = texture.create_view(&TextureViewDescriptor {
                    label: Some(
                        &(object.get_texture_handle().as_ref().to_string() + "TEXTURE_VIEW"),
                    ),
                    format: Some(TextureFormat::R32Float),
                    dimension: Some(TextureViewDimension::D3),
                    aspect: wgpu::TextureAspect::All,
                    base_mip_level: 0,
                    mip_level_count: Some(1),
                    base_array_layer: 0,
                    array_layer_count: Some(1),
                    ..Default::default()
                });
                let new_texture_detail = TextureDetail {
                    count: 1,
                    texture_view: texture_view,
                };
                self.texture_views
                    .insert(*object.get_texture_handle(), new_texture_detail.clone());
                new_texture_detail.clone()
            }
        };
    }
}
