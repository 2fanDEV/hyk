use std::{path::Path, sync::Arc};

use anyhow::Result;
use device::WGPUDevice;
use enums::BufferInput;
use instance::WGPUInstance;
use log::debug;
use shader_store::{ShaderIdentifier, ShaderStore};
use utils::pipeline_attachments::{
    color_target_state, create_vertex_state, pipeline_layout_descriptor, render_pipeline_descriptor,
};
use wgpu::{
    BlendState, BufferDescriptor, BufferUsages, Color, ColorWrites, CommandEncoder, Device, Face,
    FrontFace, MultisampleState, PipelineLayoutDescriptor, PrimitiveTopology,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, StoreOp, Surface,
    SurfaceConfiguration, TextureView,
};
use winit::window::Window;

mod device;
mod enums;
mod instance;
mod shader_store;
mod utils;
mod egui_integration;
mod ui;

pub struct FrameData {}

pub struct Core {
    pub instance: WGPUInstance,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub device: Arc<WGPUDevice>,
    pub shader_store: ShaderStore,
    pub encoder: CommandEncoder,
    pub render_pipeline: RenderPipeline,
}

impl Core {
    pub fn init(window: Arc<Window>) -> Result<Self> {
        let instance = WGPUInstance::init_instance()?;
        let window_size = window.inner_size();
        let surface = instance.create_surface(window)?;
        let device = Arc::new(WGPUDevice::create_device(&instance)?);
        let surface_capabilities = surface.get_capabilities(&instance.adapter);
        let surface_config = surface
            .get_default_config(&instance.adapter, window_size.width, window_size.height)
            .unwrap();
        let surface_format = surface_capabilities.formats;
        let mut shader_store = ShaderStore::new(device.clone());
        Self::populate_shader_store(&mut shader_store);
        let encoder = device.create_command_encoder(&Default::default());
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor         {
            label: Some("Main Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline_layout =
            device.create_pipeline_layout(&pipeline_layout_descriptor(None, &[], &[]));

        let render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor(
            Some("Main"),
            &pipeline_layout,
            create_vertex_state(shader_store.get(ShaderIdentifier::VERTEX_2D).unwrap(), &[]),
            shader_store.get(ShaderIdentifier::FRAGMENT_2D),
            PrimitiveTopology::TriangleList,
            FrontFace::Cw,
            Some(Face::Front),
            wgpu::PolygonMode::Fill,
            None,
            MultisampleState {
                count: 1,
                mask: 0,
                alpha_to_coverage_enabled: false,
            },
            &color_target_state(
                surface_config.format,
                Some(BlendState::REPLACE),
                ColorWrites::ALL,
            ),
        ));

        Ok(Self {
            instance,
            surface,
            surface_config,
            shader_store,
            device: device,
            encoder: encoder,
            render_pipeline,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            let config = &mut self.surface_config;
            config.width = width;
            config.height = height;
            self.surface.configure(&self.device, config);
        }
    }

    pub fn begin_render_pass(
        &mut self,
        label: &str,
        texture_view: TextureView,
    ) -> wgpu::RenderPass {
        let desc = RenderPassDescriptor {
            label: Some(label),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(Color::BLACK),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        };
        self.encoder.begin_render_pass(&desc)
    }

    pub fn create_buffer<T>(
        device: &Device,
        label: &str,
        usage: BufferUsages,
        mapped: bool,
        element: BufferInput<T>,
    ) -> wgpu::Buffer {
        let size = match element {
            BufferInput::Single(singular) => size_of_val::<T>(&singular),
            BufferInput::Multiple(elements) => size_of::<T>() * elements.len(),
        } as u64;
        let desc = BufferDescriptor {
            label: Some(label),
            size,
            usage,
            mapped_at_creation: mapped,
        };

        device.create_buffer(&desc)
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
            (   ShaderIdentifier::TEXTURE_FRAGMENT_2D,
                Path::new("shaders/2D_texture_fragment_shader.spv")
            )
        ];

        for (ident, path) in shader_pairs {
            shader_store.insert(ident, &path);
        }
    }
}
