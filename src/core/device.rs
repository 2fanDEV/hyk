use std::ops::Deref;

use anyhow::{anyhow, Result};
use thiserror::Error;
use wgpu::{Device, DeviceDescriptor, Features, Limits, Queue};

use super::instance::WGPUInstance;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("{0}")]
    DeviceCreationError(String),
}

#[derive(Debug, Clone)]
pub struct WGPUDevice {
    device: Device,
    pub queue: Queue,
}

impl Deref for WGPUDevice {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl WGPUDevice {
    pub fn create_device(instance: &WGPUInstance) -> Result<WGPUDevice> {
                let device_descriptor = DeviceDescriptor {
                    required_features: Features::default()
                        | Features::PUSH_CONSTANTS
                        | Features::TEXTURE_BINDING_ARRAY,
                    required_limits: Limits {
                        max_binding_array_elements_per_shader_stage: 1,
                        max_binding_array_sampler_elements_per_shader_stage: 1,
                        max_push_constant_size: 64,
                        ..Default::default()
                    },
                    ..Default::default()
                };
                let (device, queue) =
                    match pollster::block_on(instance.adapter.request_device(&device_descriptor)) {
                        Ok(res) => res,
                        Err(err) => {
                            return Err(anyhow!(DeviceError::DeviceCreationError(err.to_string())))
                        }
                    };

                Ok(Self { device, queue })
    }
}

#[cfg(test)]
mod tests {
    use crate::core::instance::WGPUInstance;

    use super::WGPUDevice;

    #[test]
    fn create_device_test() {
        let instance = WGPUInstance::init_instance().unwrap();
        let device = WGPUDevice::create_device(&instance);
        assert!(device.is_ok());
    }
}
