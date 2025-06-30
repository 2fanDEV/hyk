use std::sync::Arc;

use egui::{Context, ViewportId, WidgetText};
use egui_winit::{EventResponse, State};
use winit::{
    event::WindowEvent,
    window::{Theme, Window},
};

use super::ui::{settings_menu::SettingsMenu, Ui};


pub struct EguiIntegration {
    pub state: State,
    pub window: Arc<Window>,
}

impl EguiIntegration {
    pub fn new(window: Arc<Window>) -> Self {
        let context = Context::default();
        egui_extras::install_image_loaders(&context);
        let mut state = State::new(
            context,
            ViewportId::ROOT,
            &*window,
            Some(2.0 * window.scale_factor() as f32),
            Some(Theme::Dark),
            Some(1024 * 4),
        );
        let raw_input = state.take_egui_input(&*window);
        #[allow(irrefutable_let_patterns)]
        while let font_output = state.egui_ctx().run(raw_input.clone(), |ctx| {}) {
            if (font_output.textures_delta.is_empty()) {
                break;
            }
        }
        Self {
            state,
            window: window.clone(),
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> EventResponse {
        self.state.on_window_event(&self.window, event)
    }

    pub fn ui(&mut self, window: Arc<Window>) {
        let raw_input = self.state.take_egui_input(&*window);
        let settings_ui = SettingsMenu::create(&mut self.state, raw_input);
        //       self.state
        //         .handle_platform_output(&self.window, settings_ui.output.platform_output);
    }
}
