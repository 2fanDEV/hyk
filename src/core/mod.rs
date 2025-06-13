use std::{path::Path, sync::Arc};

use anyhow::Result;
use device::WGPUDevice;
use instance::WGPUInstance;
use shader_store::{ShaderIdentifier, ShaderStore};
use wgpu::{CommandEncoder, CommandEncoderDescriptor, RenderPassDescriptor};
use winit::window::Window;

mod device;
mod instance;
mod shader_store;
mod utils;

pub struct FrameData {

}

pub struct Core {
    pub instance: WGPUInstance,
    pub device: Arc<WGPUDevice>,
    pub shader_store: ShaderStore,
    pub encoder: CommandEncoder,
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
        let mut shader_store = ShaderStore::new(device.clone());
        Self::populate_shader_store(&mut shader_store);
        let encoder = device.create_command_encoder(&Default::default());
        Ok(Self {
            instance,
            shader_store,
            device: device,
            encoder: encoder,
        })
    }

    fn populate_shader_store(shader_store: &mut ShaderStore) {
        let shader_pairs = [
            (
                ShaderIdentifier::FRAGMENT_2D,
                Path::new("shaders/2D_fragment_shader.spv"),
            ),
            (
                ShaderIdentifier::VERTEX_2D,
                Path::new("shaders/2D_vertex_shader.spv"),
            ),
        ];

        for (ident, path) in shader_pairs {
            shader_store.insert(ident, &path);
        }
    }
}
