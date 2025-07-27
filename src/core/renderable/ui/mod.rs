use bytemuck::cast_slice;
use egui::{
    epaint::{Primitive, Vertex},
    ClippedPrimitive, Context, Id, ImageData, InnerResponse, RawInput, Response, ScrollArea,
    TextureId,
};
use egui_winit::State;
use wgpu::{
    Extent3d, Origin3d, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture, TextureAspect,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor, TextureViewDimension,
};

use crate::core::device::WGPUDevice;

pub mod settings_menu;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scissor {
    pub width: u32,
    pub height: u32,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone)]
pub struct Mesh<T> {
    pub vertices: Vec<T>,
    pub indices: Vec<u32>,
    pub texture_id: TextureId,
    pub scissor: Scissor,
}

trait UiSealed {
    fn get_texture(&self) -> Option<Texture>;
    fn texture(&mut self, texture: Texture);
    fn get_texture_view(&self) -> Option<TextureView>;
    fn texture_view(&mut self, texture_view: TextureView);
    fn get_open(&self) -> bool;
    fn open(&mut self, closed: bool);
    fn is_content_expanded_target(&self) -> bool;
    fn set_content_expanded_target(&mut self, expanded: bool);
    fn max_content_height(&mut self, max_content_height: f32);
    fn get_max_content_height(&self) -> f32;
    fn inner_ui(&self, ui: &mut egui::Ui);
    fn ui(&self, ctx: &Context) -> InnerResponse<Option<()>> {
        let inner_response = egui::Window::new("TAFAK")
            .fade_in(true)
            .fade_out(true)
            .vscroll(true)
            .resizable([true, false])
            .open(&mut self.get_open())
            .title_bar(true)
            .collapsible(true)
            .movable(true)
            .show(ctx, |ui| {
                let animation_id = Id::new("Window Collapse animation");
                let animation_progress = ctx.animate_bool_with_time(
                    animation_id,
                    self.is_content_expanded_target(),
                    0.0,
                );
                let current_height = animation_progress * self.get_max_content_height();
                if current_height >= 1.0 {
                    ScrollArea::vertical()
                        .max_height(current_height)
                        .show(ui, |ui| self.inner_ui(ui));
                }

                if animation_progress > 0.0 && animation_progress < 1.0 {
                    ctx.request_repaint()
                }
            })
            .unwrap();
        inner_response
    }
}

#[allow(private_bounds)]
pub trait Ui: UiSealed {
    fn new(device: &WGPUDevice, state: &mut State, raw_input: RawInput) -> Self;
    fn update<Vertex>(
        &mut self,
        device: &WGPUDevice,
        state: &mut State,
        raw_input: RawInput,
    ) -> Vec<Mesh<egui::epaint::Vertex>> {
        let ctx = state.egui_ctx().clone();
        if self.get_texture().is_none() && self.get_texture_view().is_none() {
            let image_data = ctx
                .run(raw_input.clone(), |ctx| {
                    self.ui(ctx);
                })
                .textures_delta
                .set[0]
                .1
                .image
                .clone();
            let (texture, texture_view) = Self::create_image_data(
                device,
                Some("Settings font"),
                TextureDimension::D2,
                image_data,
            );
            self.texture(texture);
            self.texture_view(texture_view);
        }

        let output = ctx.run(raw_input, |ctx| {
            self.ui(ctx);
        });
        let clipped_primitives = ctx.tessellate(
            output.shapes.clone(),
            ctx.native_pixels_per_point().unwrap(),
        );
        create_mesh_details(&clipped_primitives, state.egui_ctx().pixels_per_point())
    }

    fn create_image_data(
        device: &WGPUDevice,
        label: Option<&str>,
        dimension: TextureDimension,
        image_data: ImageData,
    ) -> (Texture, TextureView) {
        let image_width = image_data.width();
        let image_height = image_data.height();
        let image_size = Extent3d {
            width: image_width as u32,
            height: image_height as u32,
            depth_or_array_layers: 1,
        };
        let colors = match image_data {
            ImageData::Color(color_image) => color_image.pixels.clone(),
            ImageData::Font(font_image) => font_image.srgba_pixels(None).collect::<Vec<_>>(),
        };
        let format = TextureFormat::Rgba8Unorm;
        let usage = TextureUsages::TEXTURE_BINDING;
        let data = cast_slice(&colors) as &[u8];
        let texture = device.create_texture(&TextureDescriptor {
            label: label,
            size: image_size,
            dimension,
            format,
            usage: usage | TextureUsages::COPY_DST,
            view_formats: &[format],
            mip_level_count: 1,
            sample_count: 1,
        });
        device.queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                aspect: TextureAspect::All,
                origin: Origin3d::default(),
                mip_level: 0,
            },
            data,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(
                    ((image_width as u32 * 4) + wgpu::COPY_BYTES_PER_ROW_ALIGNMENT - 1)
                        / wgpu::COPY_BYTES_PER_ROW_ALIGNMENT
                        * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT,
                ),
                rows_per_image: Some(image_height as u32),
            },
            image_size,
        );
        let texture_view = texture.create_view(&TextureViewDescriptor {
            label,
            format: Some(format),
            dimension: Some(TextureViewDimension::D2),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: Some(1),
            base_array_layer: 0,
            array_layer_count: Some(1),
            ..Default::default()
        });
        (texture, texture_view)
    }
}

fn create_mesh_details(
    clipped_primitives: &[ClippedPrimitive],
    pixels_per_point: f32,
) -> Vec<Mesh<egui::epaint::Vertex>> {
    let mut result: Vec<Mesh<Vertex>> = vec![];
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
                let clip_min_x = (clip_rect.min.x * pixels_per_point).round() as i32;
                let clip_min_y = (clip_rect.min.y * pixels_per_point).round() as i32;
                let clip_max_x = (clip_rect.max.x * pixels_per_point).round() as i32;
                let clip_max_y = (clip_rect.max.y * pixels_per_point).round() as i32;

                // Calculate the physical extent
                let scissor_width = (clip_max_x - clip_min_x).max(0) as u32;
                let scissor_height = (clip_max_y - clip_min_y).max(0) as u32;

                // Calculate the physical offset
                let scissor_x = clip_min_x.max(0) as u32;
                let scissor_y = clip_min_y.max(0) as u32;
                let scissor = Scissor {
                    width: scissor_width,
                    height: scissor_height,
                    x: scissor_x,
                    y: scissor_y,
                };
                let mesh_details = Mesh {
                    vertices,
                    indices,
                    texture_id,
                    scissor,
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

    use crate::core::{device::WGPUDevice, instance::WGPUInstance};

    use super::{settings_menu::SettingsMenu, Ui};
    use egui::{epaint::Vertex, Context, RawInput, ViewportId};
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

    fn init() -> (impl Ui, WGPUDevice, State, RawInput) {
        let ctx = Context::default();
        let instance = WGPUInstance::init_instance().unwrap();
        let device = WGPUDevice::create_device(&instance).unwrap();
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
        (
            SettingsMenu::new(&device, &mut state, raw_input.clone()),
            device,
            state,
            raw_input,
        )
    }

    #[test]
    fn create_ui_test() {
        let (mut ui, device, mut state, raw_input) = init();
        let meshes = ui.update::<Vertex>(&device, &mut state, raw_input);
        assert!(!meshes.is_empty());
    }

    #[test]
    fn create_mesh_details_test() {
        let (mut ui, device, mut state, raw_input) = init();
        let meshes = ui.update::<Vertex>(&device, &mut state, raw_input);
        assert!(!meshes[0].vertices.is_empty());
    }
}
