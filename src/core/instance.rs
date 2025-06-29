use std::{ops::Deref, sync::Arc};

use anyhow::Result;
use wgpu::{Adapter, Instance, InstanceDescriptor, InstanceFlags, RequestAdapterOptions};

#[derive(Debug)]
pub struct WGPUInstance {
    instance: Instance,
    pub adapter: Adapter,
}

impl Deref for WGPUInstance {
    type Target = Instance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl WGPUInstance {
    pub fn init_instance() -> Result<WGPUInstance> {
        let instance_descriptor = InstanceDescriptor::from_env_or_default();
        let instance = wgpu::Instance::new(&instance_descriptor);
        let adapter =
            pollster::block_on(instance.request_adapter(&RequestAdapterOptions::default())).expect("Error while requesting adapter");
        Ok(Self {
            instance,
            adapter,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::WGPUInstance;

    #[test]
    fn init_instance_test() {
        let wgpu_instance =
            WGPUInstance::init_instance().expect("Failed to initialize instance and adapter");
        assert!(wgpu_instance.instance.generate_report().is_some())
    }
}
