use std::collections::HashMap;

use wgpu::TextureView;



#[allow(non_camel_case_types)]
pub enum TextureHandle {
    VERTEX_2D,
    VERTEX_3D,
}


pub struct ResourceManager {
    texture_views: HashMap::<TextureHandle, TextureView> 
}

impl ResourceManager {
    pub fn new() -> Self {
        Self{
            texture_views: HashMap::new()
        }
    }
}
