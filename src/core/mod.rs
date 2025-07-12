use std::{path::Path, sync::Arc};

use anyhow::{anyhow, Result};
use buffers::{ElementBuffer, MeshBuffer};
use device::WGPUDevice;
use egui::epaint::Vertex;
use egui_integration::EguiIntegration;
use enums::BufferInput;
use geometry::{BindingGroupLayoutInformation, PushConstants, VertexStateInformation};
use instance::WGPUInstance;
use sampler::create_egui_sampler;
use shader_store::{ShaderIdentifier, ShaderStore};
use ui::{settings_menu::SettingsMenu, Ui};
use utils::{
    pipeline_attachments::{color_target_state, create_vertex_state, render_pipeline_descriptor},
    push_constants::{EguiPushConstant, PushConstantType},
};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindingResource, BlendState, BufferDescriptor,
    BufferUsages, Color, ColorWrites, CommandEncoderDescriptor, Device, FrontFace,
    IndexFormat, MultisampleState, PipelineLayoutDescriptor, PresentMode, PrimitiveTopology,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, StoreOp, Surface,
    SurfaceConfiguration, TextureViewDescriptor,
};
use winit::window::Window;

pub mod buffers;
mod device;
mod egui_integration;
mod enums;
pub mod geometry;
mod instance;
mod sampler;
mod shader_store;
mod ui;
mod utils;
mod model_pipeline;

pub struct FrameData {}
pub struct Core {
    pub instance: WGPUInstance,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub device: Arc<WGPUDevice>,
    pub shader_store: ShaderStore,
    pub render_pipeline: RenderPipeline,
    pub integration: EguiIntegration,
    pub egui_buffers: Vec<MeshBuffer<Vertex>>,
    pub window: Arc<Window>,
    pub window_scale: f32,
    pub settings: SettingsMenu,
}

impl Core {
    pub fn init(window: Arc<Window>) -> Result<Self> {
        let instance = WGPUInstance::init_instance()?;
        let window_size = window.inner_size();
        let window_scale = window.scale_factor() as f32;
        let mut integration = EguiIntegration::new(window.clone());
        let device = Arc::new(WGPUDevice::create_device(&instance)?);
        /*let extent = Extent3d {
            width: 1920,
            height: 1080,
            depth_or_array_layers: 1,
        };*/
        let mut settings = integration.ui(device.clone(), window.clone());
        let raw_input = integration.state.take_egui_input(&window);
        let meshes = settings.update(&device, &mut integration.state, raw_input);
        let surface = instance.create_surface(window.clone())?;
        let egui_buffers = meshes
            .into_iter()
            .map(|mesh| {
                MeshBuffer::new(
                    ElementBuffer::new_mapped(
                        &device,
                        Some("UI_Elements"),
                        BufferUsages::VERTEX,
                        Some(mesh.scissor),
                        buffers::ElementType::VECTOR(mesh.vertices),
                    )
                    .unwrap(),
                    ElementBuffer::new_mapped(
                        &device,
                        Some("UI_Elements"),
                        BufferUsages::INDEX,
                        None,
                        buffers::ElementType::VECTOR(mesh.indices),
                    )
                    .unwrap(),
                )
            })
            .collect::<Vec<_>>();
 //       let surface_capabilities = surface.get_capabilities(&instance.adapter);
        let mut surface_config = surface
            .get_default_config(&instance.adapter, window_size.width, window_size.height)
            .unwrap();
        surface_config.present_mode = PresentMode::Fifo;
        surface.configure(&device, &surface_config);
        let mut shader_store = ShaderStore::new(device.clone());
        Self::populate_shader_store(&mut shader_store);
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Main Layout"),
            bind_group_layouts: &Vertex::binding_group_layouts(&device)
                .iter()
                .map(|mp| mp)
                .collect::<Vec<_>>(),
            push_constant_ranges: &[Vertex::push_constant_ranges::<EguiPushConstant>()],
        });

        let render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor(
            Some("Main"),
            &pipeline_layout,
            create_vertex_state(
                shader_store.get(ShaderIdentifier::VERTEX_2D).unwrap(),
                &Vertex::vertex_state(),
            ),
            shader_store.get(ShaderIdentifier::FRAGMENT_2D),
            PrimitiveTopology::TriangleList,
            FrontFace::Cw,
            None,
            wgpu::PolygonMode::Fill,
            None,
            MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            &color_target_state(
                surface_config.format,
                Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING), // Changed from REPLACE to ALPHA_BLENDING
                ColorWrites::ALL,
            ),
        ));
        Ok(Self {
            instance,
            surface,
            surface_config,
            shader_store,
            device: device,
            render_pipeline,
            integration,
            window_scale,
            egui_buffers,
            settings,
            window: window.clone()
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

    pub fn egui_pass(&mut self, label: &str) -> Result<()> {
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("EGUI Command Encoder"),
            });
        let surface_texture = self.surface.get_current_texture()?;
        let texture_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let egui_texture_view = self.settings.texture_view.clone().unwrap();

        let desc = RenderPassDescriptor {
            label: Some(label),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(Color {
                        r: 0.2,
                        g: 0.3,
                        b: 0.4,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        };

        // extra scope for encoder borrow
        {
            let mut render_pass = encoder.begin_render_pass(&desc);
            // Create test texture

            render_pass.set_pipeline(&self.render_pipeline);

            let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
                label: Some("Bind Group Test"),
                layout: &Vertex::binding_group_layouts(&self.device)[0],
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&egui_texture_view),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&create_egui_sampler(&self.device)?),
                    },
                ],
            });

            render_pass.set_bind_group(0, &bind_group, &[]);

            let width = self.surface_config.width as f32;
            let height = self.surface_config.height as f32;
            let push_constants = utils::push_constants::EguiPushConstant::new(width, height, self.window_scale);
            render_pass.set_push_constants(wgpu::ShaderStages::VERTEX, 0, &push_constants.as_raw());
            render_pass.set_viewport(0.0, 0.0, width, height, 0.0, 1.0);

            for mesh_buffer in &self.egui_buffers {
                render_pass.set_vertex_buffer(0, mesh_buffer.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(mesh_buffer.index_buffer.slice(..), IndexFormat::Uint32);
                let scissor = mesh_buffer.vertex_buffer.scissor.unwrap();
                render_pass.set_scissor_rect(scissor.x, scissor.y, scissor.width, scissor.height);
                let elements = match &mesh_buffer.index_buffer.elements {
                    buffers::ElementType::VECTOR(items) => items,
                    buffers::ElementType::SINGLE_ELEMENT(_) => {
                        return Err(anyhow!("In this pass there should never be a single item"))
                    }
                };
                render_pass.draw_indexed(0..elements.iter().len() as u32, 0, 0..1);
            }
        }

        let command_buffer = encoder.finish();
        self.device.queue.submit(std::iter::once(command_buffer));
        surface_texture.present();
        Ok(())
    }

    pub fn update_egui(&mut self) {
        self.integration
            .update_ui(&mut self.settings, self.device.clone());
        let raw_input = self.integration.state.take_egui_input(&self.window);
        let meshes = self.settings.update(&self.device, &mut self.integration.state, raw_input);
        self.egui_buffers = meshes
            .into_iter()
            .map(|mesh| {
                MeshBuffer::new(
                    ElementBuffer::new_mapped(
                        &self.device,
                        Some("UI_Elements"),
                        BufferUsages::VERTEX,
                        Some(mesh.scissor),
                        buffers::ElementType::VECTOR(mesh.vertices),
                    )
                    .unwrap(),
                    ElementBuffer::new_mapped(
                        &self.device,
                        Some("UI_Elements"),
                        BufferUsages::INDEX,
                        None,
                        buffers::ElementType::VECTOR(mesh.indices),
                    )
                    .unwrap(),
                )
            })
            .collect::<Vec<_>>();
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
        let mut counter = 0;
        let shader_pairs = [
            (
                ShaderIdentifier::FRAGMENT_2D,
                Some("2D_FRAGMENT"),
                Path::new("/Users/zapzap/Projects/hyk/shaders/2D_fragment_shader.spv"),
            ),
            (
                ShaderIdentifier::VERTEX_2D,
                Some("2D_VERTEX_SHADER"),
                Path::new("/Users/zapzap/Projects/hyk/shaders/2D_vertex_shader.spv"),
            ),
            (
                ShaderIdentifier::TEXTURE_FRAGMENT_2D,
                Some("TEXTURE_FRAGMENT"),
                Path::new("/Users/zapzap/Projects/hyk/shaders/2D_texture_fragment_shader.spv"),
            ),
        ];

        for (ident, label, path) in shader_pairs {
            let unnamed = Some("UNNAMED_".to_string() + counter.to_string().as_str());
            if label.is_none() {
                counter += 1;
            }
            shader_store.insert(ident, label.or_else(|| unnamed.as_deref()), &path);
        }
    }
}
