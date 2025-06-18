use std::sync::Arc;

use egui::{epaint::{Primitive, Vertex}, ClippedPrimitive, Context, FullOutput, InnerResponse, RawInput, Response, TextureId, WidgetText};
use egui_winit::State;
use wgpu::{Buffer, BufferDescriptor, Device};

pub trait Ui {
    fn create(state: &State, raw_input: RawInput) -> Self;
    fn create_buffer(&self, device: &Device);
}

pub struct MeshDetails {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    texture_id: TextureId,
}

pub struct Settings {
    output: FullOutput,
    primitives: Vec<ClippedPrimitive>,

}

impl Ui for Settings {
    fn create(state: &State, raw_input: RawInput) -> Self {
        let output = state.egui_ctx().run(raw_input, |ctx| {
            egui::Window::new(WidgetText::default().strong())
                .open(&mut true)
                .vscroll(true)
                .resizable(true)
                .show(ctx, |ui| ui.label("Hello world!"));
        });
        let primitives = state
            .egui_ctx()
            .tessellate(output.shapes.clone(), output.pixels_per_point);
        Self { output, primitives }
    }

    fn create_buffer(&self, device: &Device) {
          }
}

fn create_mesh_details(clipped_primitives: Vec<ClippedPrimitive>) -> Vec<MeshDetails> {
    let mut result: Vec<MeshDetails> = vec![];
    for ClippedPrimitive {
        primitive,
        clip_rect
    } in clipped_primitives {
        match primitive {
            Primitive::Mesh(mesh) => {
              let vertices = mesh.vertices;
              let indices = mesh.indices;
              let texture_id = mesh.texture_id;
              let mesh_details = MeshDetails {
                  vertices,
                  indices,
                  texture_id
              };
              result.push(mesh_details);
            },
            Primitive::Callback(paint_callback) => todo!(),
        }
    };
    result
}

#[cfg(test)]
mod tests {
    use egui::{Context, ViewportId};
    use egui_winit::State;
    use mockall::mock;
    use wgpu::rwh::HasDisplayHandle;
    use winit::window::Theme;
    use super::{Settings, Ui};

    fn create_mesh_details_test() {
        let ctx = Context::default();
        mock!(
            pub HasDisplayHandle {}
            impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for &H {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        (**self).display_handle()
    }
}
        );
        let display = MockHasDisplayHandle::new();
        display.checkpoint();
        let state = State::new(ctx, ViewportId::ROOT, &display, Some(0.0), Some(Theme::Dark), Some(0));
        Settings::create(, raw_input)

    }
}
