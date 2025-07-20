use std::{sync::Arc, thread::sleep, time};

use muda::dpi::PhysicalSize;
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

const POLL_SLEEP_TIME: std::time::Duration = time::Duration::from_millis(1);

#[allow(warnings)]
impl ApplicationHandler for App {
    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window.as_ref().unwrap().request_redraw();
        match event_loop.control_flow() {
            winit::event_loop::ControlFlow::Poll => {
                sleep(POLL_SLEEP_TIME);
            }
            _ => todo!(),
        }
    }


    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes =
            WindowAttributes::default().with_inner_size(PhysicalSize::new(3840, 1920));
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
