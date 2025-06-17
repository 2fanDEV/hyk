use egui::{Context, FullOutput, InnerResponse, RawInput, Response, WidgetText};
use egui_winit::State;

pub trait Ui {
    fn create(ctx: &Context, res: impl FnMut(&Option<InnerResponse<Option<Response>>>) -> FullOutput) -> FullOutput;
}

pub struct Settings {
    output: FullOutput,
}

impl Ui for Settings {
    fn create(ctx: &Context, mut res: impl FnMut(&Option<InnerResponse<Option<Response>>>) -> FullOutput) -> FullOutput {
        let x = egui::Window::new(WidgetText::default().strong())
            .open(&mut true)
            .vscroll(true)
            .resizable(true)
            .show(ctx, |ui| ui.label("Hello world!"));
        res(&x)
    }
}
