use egui::{
    epaint::{Primitive, Vertex}, ClippedPrimitive, RawInput, Shape, TextureId
};
use egui_winit::State;
pub mod settings_menu;

#[derive(Debug)]
pub struct Meshes {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub texture_id: TextureId,
}

pub trait Ui {
    fn create(state: &mut State, raw_input: RawInput) -> Self;
    fn primitives(&self) -> &[ClippedPrimitive];
    fn meshes(&self) -> Vec<Meshes>;
}

fn create_mesh_details(clipped_primitives: &[ClippedPrimitive]) -> Vec<Meshes> {
    let mut result: Vec<Meshes> = vec![];
    for ClippedPrimitive {
        primitive,
        clip_rect,
    } in clipped_primitives
    {
        match primitive {
            Primitive::Mesh(mesh) => {
                let vertices = mesh.vertices.clone();
                let indices = mesh.indices.clone();
                let texture_id = mesh.texture_id;
                let mesh_details = Meshes {
                    vertices,
                    indices,
                    texture_id,
                };
                result.push(mesh_details);
            }
            Primitive::Callback(paint_callback) => todo!(),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{settings_menu::SettingsMenu, Ui};
    use egui::{Context, RawInput, ViewportId};
    use egui_winit::State;
    use mockall::mock;
    use wgpu::rwh::{DisplayHandle, HasDisplayHandle};
    use winit::window::Theme;

    mock!(
        pub HasDisplayHandle {}
    );
    impl HasDisplayHandle for MockHasDisplayHandle {
        fn display_handle(&self) -> Result<wgpu::rwh::DisplayHandle<'_>, wgpu::rwh::HandleError> {
            Ok(DisplayHandle::uikit())
        }
    }

    fn init() -> impl Ui {
        let ctx = Context::default();

        let display = MockHasDisplayHandle::new();
        let mut state = State::new(
            ctx,
            ViewportId::ROOT,
            &display,
            Some(0.0),
            Some(Theme::Dark),
            Some(0),
        );
        let raw_input = RawInput::default();
        SettingsMenu::create(&mut state, raw_input)
    }

    #[test]
    fn create_ui_test() {
        let ui = init();
        assert!(!ui.primitives().is_empty());
    }

    #[test]
    fn create_mesh_details_test() {
        let ui = init();
        let meshes = ui.meshes();
        assert!(!meshes[0].indices.is_empty());
        assert!(!meshes[0].vertices.is_empty());
    }
}
