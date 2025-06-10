use std::{ops::Deref, sync::Arc};

use anyhow::Result;
use wgpu::{Adapter, Instance, InstanceDescriptor, RequestAdapterOptions, Surface};
use winit::window::Window;

#[derive(Default, Debug)]
pub struct WGPUInstance {
    instance: Instance,
    pub adapter: Option<Adapter>,
    pub surface: Option<Surface<'static>>,
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
            pollster::block_on(instance.request_adapter(&RequestAdapterOptions::default()));
        Ok(Self {
            instance,
            adapter: adapter.ok(),
            surface: None,
        })
    }

    pub fn init_surface(&mut self, window: Window) {
        self.surface = self.instance.create_surface(window).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::WGPUInstance;


    #[test]
    fn init_instance_test() {
        let wpgu_instance =
            WGPUInstance::init_instance().expect("Failed to initialize instance and adapter");
        assert!(wpgu_instance.adapter.is_some());
    }

    #[test]
    fn init_surface() {
        let wpgu_instance = WGPUInstance::init_instance().unwrap();
        assert!(wpgu_instance.surface.is_none());
        // init window
        // wpgu_instance.init_surface(handle);
        // assert!(wpgu_instance.surface.is_some());
    }
}
