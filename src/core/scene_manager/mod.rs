use std::sync::Arc;

use render_pipeline::ModelRenderPipeline;
use wgpu::SurfaceConfiguration;

use super::{buffers::MeshBuffer, device::WGPUDevice, geometry::vertex3d::Vertex3D, shader_store::ShaderStore, ui::Mesh};

mod parser;
mod render_pipeline;

#[derive(Debug)]
pub struct SceneManager {
    mesh_buffers: Vec<MeshBuffer<Vertex3D>>,
    render_pipeline: ModelRenderPipeline
}

impl SceneManager { 
    pub fn new(device: Arc<WGPUDevice>, shader_store: &ShaderStore, surface_config: SurfaceConfiguration) -> Self {
        let model_render_pipeline = ModelRenderPipeline::new(device, , vertex_shader, surface_config, fragment_shader);

        Self {
            mesh_buffers: vec![],
            render_pipeline: model_render_pipeline
        }
    }

    fn update(&mut self) {
        
    }

    pub fn render(&mut self) {
                   
    }

}
