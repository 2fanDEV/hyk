use std::sync::Arc;

use egui::ahash::{HashMap, HashMapExt};
use parser::MeshLoader;
use render_pipeline::ModelRenderPipeline;
use wgpu::{hal::Device, CommandEncoder, Operations, RenderPassColorAttachment, RenderPassDescriptor, Surface, SurfaceConfiguration};

use super::{
    device::WGPUDevice,
    geometry::vertex3d::Vertex3D,
    renderable::{identifier::AssetIdentifier, objects::donut::Donut, Renderable},
    shader_store::{ShaderIdentifier, ShaderStore},
    utils::push_constants::ScenePushConstant,
};

pub mod parser;
mod render_pipeline;

#[derive(Debug)]
pub struct SceneManager {
    render_pipeline: ModelRenderPipeline,
    mesh_loader: Arc<MeshLoader>,
    device: Arc<WGPUDevice>,
    renderables: HashMap<AssetIdentifier, Box<dyn Renderable>>,
    loaded_renderables: Vec<Box<dyn Renderable>>,
}

impl SceneManager {
    pub fn new(
        device: Arc<WGPUDevice>,
        mesh_loader: Arc<MeshLoader>,
        shader_store: &ShaderStore,
        surface_config: &SurfaceConfiguration,
    ) -> Self {
        let vertex_3d_shader = shader_store.get(ShaderIdentifier::VERTEX_3D).unwrap();
        let fragment_3d_shader = shader_store.get(ShaderIdentifier::FRAGMENT_3D);
        let model_render_pipeline = ModelRenderPipeline::new::<Vertex3D, ScenePushConstant>(
            device.clone(),
            Some("SceneRenderer"),
            vertex_3d_shader,
            surface_config,
            fragment_3d_shader,
        );
        let mut renderables: HashMap<AssetIdentifier, Box<dyn Renderable>> = HashMap::new();
        renderables.insert(AssetIdentifier::DONUT, Box::new(Donut::new(mesh_loader.clone(), Some("DONUT"), device.clone())));
        Self {
            renderables,
            loaded_renderables: vec![],
            render_pipeline: model_render_pipeline,
            mesh_loader,
            device,
        }
    }

    fn update(&mut self) {

    }

    pub fn render(&mut self, encoder: &CommandEncoder, surface: &Surface) {
/*&        let txt = surface.get_current_texture();
        let render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("SceneManager Render Pass"),
            color_attachments: &[Option<RenderPassColorAttachment {
                view: ,
                resolve_target: None,
                ops: Operations {
                  load: wgpu::LoadOp::Load,
                  store: wgpu::StoreOp::Store
                }
            }],
            depth_stencil_attachment: None,
            ..Default::default()
        }); */
    }

    pub fn add_renderable(&mut self, id: AssetIdentifier, renderable: Box<dyn Renderable>) {
        self.renderables.insert(id, renderable);
    }

    pub fn remove_renderable(&mut self, id: &AssetIdentifier) -> Option<Box<dyn Renderable>> {
        self.renderables.remove(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_new_scene_manager_initial_state() {
        // TODO: Provide real or minimal instances for these types if possible.
        let device = todo!("Provide a minimal Arc<WGPUDevice> for testing");
        let mesh_loader = todo!("Provide a minimal Arc<MeshLoader> for testing");
        let shader_store = todo!("Provide a minimal ShaderStore for testing");
        let surface_config = todo!("Provide a minimal SurfaceConfiguration for testing");

        let manager = SceneManager::new(
            device,
            mesh_loader,
            &shader_store,
            &surface_config,
        );

        assert_eq!(manager.renderables.len(), 1);
        assert!(manager.renderables.contains_key(&AssetIdentifier::DONUT));
        assert!(manager.loaded_renderables.is_empty());
    }
}
