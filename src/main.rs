use hyk::app::App;
use log::{debug, LevelFilter};
use winit::event_loop::EventLoop;

fn main() {

    let mut app = App::default();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .filter_module("naga", LevelFilter::Warn)
        .try_init()
        .unwrap();
    debug!("Event loop created!");
    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let guard = tokio_runtime.enter();
    debug!("Tokio runtime started!");
    event_loop.run_app(&mut app).unwrap();
}
