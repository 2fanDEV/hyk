use std::{ops::Deref, sync::Arc};

use wgpu::{
    BlendState, ColorWrites, FrontFace, MultisampleState, PipelineLayoutDescriptor,
    PrimitiveTopology, RenderPipeline, ShaderModule,
    SurfaceConfiguration,
};

use crate::core::{
    device::WGPUDevice,
    geometry::{
        BindingGroupLayoutInformation, PushConstants, VertexStateInformation,
    },
    utils::pipeline_attachments::{
        color_target_state, create_vertex_state, render_pipeline_descriptor,
    },
};

#[derive(Debug)]
pub struct ModelRenderPipeline {
    pipeline: RenderPipeline,
}

impl Deref for ModelRenderPipeline {
    type Target = RenderPipeline;

    fn deref(&self) -> &Self::Target {
        &self.pipeline
    }
}

impl ModelRenderPipeline {
    pub fn new<GeomType, PushConstantType>(
        device: Arc<WGPUDevice>,
        label: Option<&str>,
        vertex_shader: &ShaderModule,
        surface_config: &SurfaceConfiguration,
        fragment_shader: Option<&ShaderModule>,
    ) -> Self
    where
        GeomType: BindingGroupLayoutInformation + VertexStateInformation + PushConstants,
    {
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: label,
            bind_group_layouts: &GeomType::binding_group_layouts(&device)
                .iter()
                .map(|bgl| bgl)
                .collect::<Vec<_>>(),
            push_constant_ranges: &[GeomType::push_constant_ranges::<PushConstantType>()],
        });

        let render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor(
            label,
            &pipeline_layout,
            create_vertex_state(vertex_shader, &GeomType::vertex_state()),
            fragment_shader,
            PrimitiveTopology::TriangleList,
            FrontFace::Cw,
            None, wgpu::PolygonMode::Fill,
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

        Self {
            pipeline: render_pipeline,
        }
    }
}
