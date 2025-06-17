use std::sync::Arc;

use egui::{Context, ViewportId};
use egui_winit::State;
use winit::window::{self, Theme, Window};

use super::ui::{Settings, Ui};

pub struct EguiIntegration {
    state: State,
    window: Arc<Window>,
}

impl EguiIntegration {
    fn new(window: Arc<Window>) -> Self {
        let mut ctx = Context::default();
        egui_extras::install_image_loaders(&ctx);
        let state = State::new(
            ctx.clone(),
            ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            Some(Theme::Dark),
            Some(1024 * 4),
        );

        let x = || ctx.clone();
        Self {
            state,
            window: window.clone(),
        }
    }
    fn ui(&mut self, ctx: &Context) {
        let raw_input = self.state.take_egui_input(&self.window);
        let settings_ui = Settings::create(ctx, |res| {
            self.state.egui_ctx().run(raw_input.clone(), |ctx| {
                res;
            })
        });
    }
}
