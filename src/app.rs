use std::sync::Arc;

use muda::dpi::LogicalSize;
use winit::{
    application::ApplicationHandler,
    window::{self, Window, WindowAttributes},
};

use crate::renderer::Renderer;

#[derive(Default)]
pub struct App {
    renderer: Option<Renderer>,
}

#[allow(warnings)]
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes =
            WindowAttributes::default().with_inner_size(LogicalSize::new(3840, 2160));
        let window = event_loop.create_window(window_attributes).ok().unwrap();
        self.renderer = Some(Renderer::new(window).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
    }
}
