use std::num::NonZero;

use wgpu::{
    BindGroupLayout, BlendState, ColorTargetState, ColorWrites, DepthStencilState, Face,
    FragmentState, FrontFace, MultisampleState, PipelineCache, PipelineCompilationOptions,
    PipelineLayout, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    PushConstantRange, RenderPipelineDescriptor, ShaderModule, SurfaceConfiguration, TextureFormat,
    VertexState,
};

pub fn render_pipeline_descriptor<'a>(
    label: Option<&'a str>,
    layout: &'a PipelineLayout,
    vertex_shader: &'a ShaderModule,
    fragment_shader: Option<&'a ShaderModule>,
    topology: PrimitiveTopology,
    front_face: FrontFace,
    cull_mode: Option<Face>,
    polygon_mode: PolygonMode,
    depth_stencil: Option<DepthStencilState>,
    multisample: MultisampleState,
    format: TextureFormat,
    color_target_states: &'a [Option<ColorTargetState>],
) -> RenderPipelineDescriptor<'a> {
    RenderPipelineDescriptor {
        label,
        layout: Some(layout),
        vertex: VertexState {
            entry_point: Some("main"),
            compilation_options: PipelineCompilationOptions::default(),
            buffers: &[],
            module: vertex_shader,
        },
        fragment: if fragment_shader.is_some() {
            Some(FragmentState {
                entry_point: Some("main"),
                module: fragment_shader.unwrap(),
                compilation_options: PipelineCompilationOptions::default(),
                targets: color_target_states,
            })
        } else {
            None
        },
        primitive: PrimitiveState {
            topology,
            strip_index_format: Some(wgpu::IndexFormat::Uint32),
            front_face: front_face,
            cull_mode: cull_mode,
            unclipped_depth: false,
            polygon_mode: polygon_mode,
            conservative: false,
        },
        depth_stencil,
        multisample,
        multiview: NonZero::new(1),
        cache: None,
    }
}

pub fn pipeline_layout_descriptor<'a>(
    label: Option<&'a str>,
    bind_group_layouts: &'a [&'a BindGroupLayout],
    push_constant_ranges: &'a [PushConstantRange],
) -> PipelineLayoutDescriptor<'a> {
    PipelineLayoutDescriptor {
        label,
        bind_group_layouts,
        push_constant_ranges,
    }
}

pub fn color_target_state(
    format: TextureFormat,
    blend: Option<BlendState>,
    write_mask: ColorWrites,
) -> Vec<Option<ColorTargetState>> {
    vec![Some(ColorTargetState {
        format,
        blend,
        write_mask,
    })]
}
