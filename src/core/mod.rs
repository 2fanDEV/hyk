use std::sync::Arc;

use anyhow::Result;
use device::WGPUDevice;
use instance::WGPUInstance;
use shader_store::ShaderStore;
use winit::window::Window;

mod device;
mod instance;
mod shader_store;

pub struct Core {
    instance: WGPUInstance,
    device: Arc<WGPUDevice>,
    shader_store: ShaderStore,
}

impl Core {
    pub fn init(window: Window) -> Result<Self> {
        let mut instance = WGPUInstance::init_instance()?;
        instance.init_surface(window);
        let device = Arc::new(WGPUDevice::create_device(&instance)?);
        let surface_capabilities = instance
            .surface
            .as_ref()
            .unwrap()
            .get_capabilities(instance.adapter.as_ref().unwrap());
        let surface_format = surface_capabilities.formats;

        Ok(Self {
            instance,
            shader_store: ShaderStore::new(device.clone()),
            device: device,
        })
    }
}
