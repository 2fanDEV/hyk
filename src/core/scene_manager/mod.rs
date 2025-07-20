use std::sync::Arc;

use egui::ahash::{HashMap, HashMapExt};
use render_pipeline::ModelRenderPipeline;
use wgpu::SurfaceConfiguration;

use super::{
    device::WGPUDevice,
    geometry::vertex3d::Vertex3D,
    renderable::{identifier::AssetIdentifier, Renderable},
    shader_store::{ShaderIdentifier, ShaderStore},
    utils::push_constants::ScenePushConstant,
};

mod parser;
mod render_pipeline;

#[derive(Debug)]
pub struct SceneManager {
    render_pipeline: ModelRenderPipeline,
    renderable: HashMap<AssetIdentifier, Box<dyn Renderable>>,
    loaded_renderables: Vec<Box<dyn Renderable>>,
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
            renderable: HashMap::new(),
            loaded_renderables: vec![],
            render_pipeline: model_render_pipeline,
        }
    }

    fn update(&mut self) {
        
    }

    pub fn render(&mut self) {}

    pub fn add_renderable(&mut self, id: AssetIdentifier, renderable: Box<dyn Renderable>) {
        self.renderable.insert(id, renderable);
    }

    pub fn remove_renderable(&mut self, id: &AssetIdentifier) -> Option<Box<dyn Renderable>> {
        self.renderable.remove(id)
    }
}
