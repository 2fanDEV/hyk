use std::sync::Arc;

use egui::ahash::HashMap;
use render_pipeline::ModelRenderPipeline;
use wgpu::SurfaceConfiguration;

use super::{
    buffers::MeshBuffer, device::WGPUDevice, geometry::{vertex3d::Vertex3D, PushConstants}, renderable::Renderable, shader_store::{ShaderIdentifier, ShaderStore}, ui::Mesh, utils::push_constants::ScenePushConstant
};

mod parser;
mod render_pipeline;

#[derive(Debug)]
pub struct SceneManager {
    mesh_buffers: Vec<MeshBuffer<Vertex3D>>,
    render_pipeline: ModelRenderPipeline,
    objects: HashMap<RenderIdentifier, Box<dyn Renderable>>
}

impl SceneManager {
    pub fn new(
        device: Arc<WGPUDevice>,
        shader_store: &ShaderStore,
        surface_config: &SurfaceConfiguration,
    ) -> Self {
        let vertex_3d_shader = shader_store.get(ShaderIdentifier::VERTEX_3D).unwrap();
        let fragment_3d_shader = shader_store.get(ShaderIdentifier::FRAGMENT_3D);
        let model_render_pipeline = ModelRenderPipeline::new::<Vertex3D, ScenePushConstant>(
            device,
            Some("SceneRenderer"),
            vertex_3d_shader,
            surface_config,
            fragment_3d_shader,
        );

        Self {
            mesh_buffers: vec![],
            render_pipeline: model_render_pipeline,
        }
    }

    

    fn update(&mut self) {}

    pub fn render(&mut self) {

    }
}
