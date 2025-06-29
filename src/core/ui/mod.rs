use egui::{
    epaint::{Primitive, Vertex}, ClippedPrimitive, RawInput, Shape, TextureId, ViewportId
};
use egui_winit::State;
use log::debug;


pub trait Ui {
    fn create(state: &mut State, raw_input: RawInput) -> Self;
    fn primitives(&self) -> &[Vec<ClippedPrimitive>];
    fn meshes(&self) -> MeshDetails;
}

#[derive(Debug)]
pub struct MeshDetails {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    texture_id: TextureId,
}

pub struct Settings {
    primitives: Vec<Vec<ClippedPrimitive>>,
}

impl Ui for Settings {
    fn create(state: &mut State, raw_input: RawInput) -> Self {
        let ctx = state.egui_ctx();
        println!("TEST: {:p}", ctx);
        let mut clipped_primitives: Vec<Vec<ClippedPrimitive>> = Vec::new();
        #[allow(irrefutable_let_patterns)]
        while let output = state.egui_ctx().run(raw_input.clone(), |ctx| {
            egui::Window::new("Tiesto")
                .title_bar(true)
                .vscroll(true)
                .resizable(true)
                .open(&mut true)
                .show(ctx, |ui| {
                    ui.label("Hello World");
                    if ui.button("Test").clicked() {
                        debug!("Test button");
                    }
                }).unwrap();
        }) {
            if !output.shapes.iter().find(|pred| !pred.shape.eq(&Shape::Noop)).is_some(){
                continue;
            }
            clipped_primitives.push(ctx.tessellate(output.shapes, ctx.pixels_per_point()));
            break;
        }
        Self {
            primitives: clipped_primitives,
        }
    }
    
    fn meshes(&self) -> MeshDetails {
        MeshDetails {
            vertices: vec![],
            indices: vec![],
            texture_id: egui::TextureId::Managed(0)
        }       
    }

    fn primitives(&self) -> &[Vec<ClippedPrimitive>] {
       &self.primitives
    }
}

fn create_mesh_details(clipped_primitives: Vec<ClippedPrimitive>) -> Vec<MeshDetails> {
    let mut result: Vec<MeshDetails> = vec![];
    for ClippedPrimitive {
        primitive,
        clip_rect,
    } in clipped_primitives
    {
        match primitive {
            Primitive::Mesh(mesh) => {
                let vertices = mesh.vertices;
                let indices = mesh.indices;
                let texture_id = mesh.texture_id;
                let mesh_details = MeshDetails {
                    vertices,
                    indices,
                    texture_id,
                };
                result.push(mesh_details);
            }
            Primitive::Callback(paint_callback) => todo!(),
        }
    }
    println!("TEST: {result:?}");
    result
}

#[cfg(test)]
mod tests {
    use super::{Settings, Ui};
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
        Settings::create(&mut state, raw_input)
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
        assert!(!meshes.indices.is_empty());
        assert!(!meshes.vertices.is_empty());
    }
}
