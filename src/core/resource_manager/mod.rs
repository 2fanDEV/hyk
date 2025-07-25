use std::collections::HashMap;

use wgpu::TextureView;

use super::renderable::Renderable;

#[allow(non_camel_case_types)]
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd)]
pub enum TextureHandle {
    VERTEX_2D,
    VERTEX_3D,
    CUSTOM_HANDLE(String)
}

pub struct TextureDetail {
    count: u32,
    texture_view: TextureView
}

pub struct ResourceManager {
    texture_views: HashMap::<TextureHandle, TextureDetail> 
}

impl ResourceManager {
    pub fn new() -> Self {
        Self{
            texture_views: HashMap::new()
        }
    }

    pub fn get_or_create(&self, object: &dyn Renderable) {
        let texture_detail = self.texture_views.get(object.get_texture_handle());
        return match texture_detail {
            Some(texture_detail) => texture_detail,
            None => {
                match {
                }
            },
        } 
         
    } 


}
