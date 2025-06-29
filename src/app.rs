use std::sync::Arc;

use muda::dpi::LogicalSize;
use winit::{
    application::ApplicationHandler,
    window::{Window, WindowAttributes},
};

use crate::renderer::Renderer;

#[derive(Default)]
pub struct App {
    renderer: Option<Renderer>,
    window: Option<Arc<Window>>
}

#[allow(warnings)]
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes =
            WindowAttributes::default().with_inner_size(LogicalSize::new(3840, 2160));
        let window = Arc::new(event_loop.create_window(window_attributes).ok().unwrap());
        self.window =  Some(window.clone());
        self.renderer = Some(Renderer::new(window.clone()).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.renderer.as_mut().unwrap().core.integration.input(&event);
        self.renderer.as_mut().unwrap().draw();
    }
}
