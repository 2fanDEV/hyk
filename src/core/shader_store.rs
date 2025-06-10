use std::{collections::HashMap, path, sync::Arc};

use wgpu::{wgc::id::markers::ShaderModule, Device, ShaderModuleDescriptor, ShaderSource};

use super::device::WGPUDevice;

#[derive(Debug)]
pub struct ShaderStore {
    device: Arc<WGPUDevice>,
    map: HashMap<&'static str,ShaderModule>
}

impl ShaderStore {
    
    pub fn new(device: Arc<WGPUDevice>) -> ShaderStore {
        Self {
            device: device.clone(),
            map: HashMap::new()
        }
    }

    pub fn insert(&mut self, name: &str, path: &str) {
        /* let create_shader_module = self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some(name),
        });*/
    }
}
