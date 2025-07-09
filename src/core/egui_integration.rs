use std::sync::Arc;

use egui::{Context, ViewportId};
use egui_winit::{EventResponse, State};
use winit::{
    event::WindowEvent,
    window::{Theme, Window},
};

use super::{
    device::WGPUDevice,
    ui::{settings_menu::SettingsMenu, Ui},
};

pub struct EguiIntegration {
    pub state: State,
    pub window: Arc<Window>,
}

impl EguiIntegration {
    pub fn new(window: Arc<Window>) -> Self {
        let context = Context::default();
        egui_extras::install_image_loaders(&context);
        let state = State::new(
            context,
            ViewportId::ROOT,
            &*window,
            Some(2.0 * window.scale_factor() as f32),
            Some(Theme::Dark),
            Some(1024 * 4),
        );
        Self {
            state,
            window: window.clone(),
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> EventResponse {
        self.state.on_window_event(&self.window, event)
    }

    pub fn ui(&mut self, device: Arc<WGPUDevice>, window: Arc<Window>) -> SettingsMenu {
        let raw_input = self.state.take_egui_input(&*window);
        let settings_ui = SettingsMenu::new(&device, &mut self.state, raw_input);
        settings_ui
    }

    pub fn update_ui(&mut self, settings: &mut SettingsMenu, device: Arc<WGPUDevice>)  {
        let raw_input = self.state.take_egui_input(&self.window);
        settings.update(&device, &mut self.state, raw_input);
    }
}
