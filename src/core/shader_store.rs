use std::{collections::HashMap, fs, path::Path, sync::Arc};

use anyhow::Result;
use wgpu::{
    include_spirv, include_spirv_raw,
    util::{make_spirv, make_spirv_raw},
    ShaderModule, ShaderModuleDescriptor, ShaderSource,
};

use super::device::WGPUDevice;

#[allow(unused, non_camel_case_types)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd)]
pub enum ShaderIdentifier {
    FRAGMENT_2D,
    TEXTURE_FRAGMENT_2D,
    VERTEX_2D,
    VERTEX_SCENE_DATA,
    FRAGMENT_SCENE_DATA,
}

#[allow(unused)]
#[derive(Debug)]
pub struct ShaderStore {
    device: Arc<WGPUDevice>,
    map: HashMap<ShaderIdentifier, ShaderModule>,
}

impl ShaderStore {
    pub fn new(device: Arc<WGPUDevice>) -> ShaderStore {
        Self {
            device: device.clone(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: ShaderIdentifier, path: &Path) {
        let bytes = fs::read(path).unwrap();
        let path_bytes = make_spirv_raw(&bytes);
        let spir_v = self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Test"),
            source: ShaderSource::SpirV(path_bytes),
        });

        self.map.insert(name, spir_v);
    }

    pub fn contains(&mut self, identifier: ShaderIdentifier) -> bool {
        self.map.keys().find(|key| key.eq(&&identifier)).is_some()
    }

    pub fn get(&self, identifier: ShaderIdentifier) -> Option<&ShaderModule> {
        self.map.get(&identifier)
    }
}

#[cfg(test)]
mod tests {
    use wgpu::include_spirv;

    use super::{ShaderIdentifier, ShaderStore};
    use crate::core::{device::WGPUDevice, instance::WGPUInstance};
    use std::{path::Path, sync::Arc};

    #[test]
    fn test_shader_store() {
        let instance = WGPUInstance::init_instance().unwrap();
        let device = Arc::new(WGPUDevice::create_device(&instance).unwrap());
        let mut shader_store = ShaderStore::new(device.clone());
/*        shader_store.insert(
            ShaderIdentifier::FRAGMENT_2D,
            include_spirv!(""),
        );
        let contains = shader_store.contains(ShaderIdentifier::FRAGMENT_2D);
        assert_eq!(contains, true); **/
    }
}
