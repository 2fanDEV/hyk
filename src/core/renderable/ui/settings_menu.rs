use std::cmp::max;

use egui::RawInput;
use egui_winit::State;
use log::debug;
use wgpu::{Texture, TextureView};

use crate::core::{device::WGPUDevice, utils::ranged::Ranged};

use super::{Ui, UiSealed};

pub struct SettingsMenu {
    pub volume: Ranged<u8>,
    pixels_per_point: f32,
    texture: Option<Texture>,
    pub texture_view: Option<TextureView>,
    open: bool,
    is_content_expanded_target: bool,
    max_content_height: f32, //
}


impl Ui for SettingsMenu {
    fn new(device: &WGPUDevice, state: &mut State, raw_input: RawInput) -> Self {
        Self {
            volume: Ranged::new(50u8, 0, 100).unwrap(),
            texture: None,
            texture_view: None,
            open: true,
            pixels_per_point: state.egui_ctx().pixels_per_point(),
            is_content_expanded_target: true,
            max_content_height: 1.0,
        }
    }

}

impl UiSealed for SettingsMenu {
    fn get_texture(&self) -> Option<Texture> {
        self.texture.clone()
    }

    fn texture(&mut self, texture: Texture) {
        self.texture = Some(texture);
    }

    fn get_texture_view(&self) -> Option<TextureView> {
        self.texture_view.clone()
    }

    fn texture_view(&mut self, texture_view: TextureView) {
        self.texture_view = Some(texture_view);
    }

    fn get_open(&self) -> bool {
        self.open
    }

    fn open(&mut self, closed: bool) {
        self.open = closed;
    }

    fn is_content_expanded_target(&self) -> bool {
        self.is_content_expanded_target
    }

    fn set_content_expanded_target(&mut self, expanded: bool) {
        self.is_content_expanded_target = expanded;
    }

    fn max_content_height(&mut self, max_content_height: f32) {
        self.max_content_height = max_content_height;
    }

    fn get_max_content_height(&self) -> f32 {
        self.max_content_height
    }

    fn inner_ui(&self, ui: &mut egui::Ui) {
        ui.label("Hello world!");
        if ui.button("Click me").clicked() {
            debug!("CLICKED");
        }
        /*  ui.image(egui::include_image!(
            "/Users/zapzap/Projects/piplup/shaders/ferris.png"
        )); */
        if ui.button("WHAT THE HEEEEEEELLL").clicked() {
            debug!("WHAT THE HEEEEELL");
        }
    }
}
