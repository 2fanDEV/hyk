use egui::{Context, FullOutput, InnerResponse, Response, WidgetText};
use egui_winit::State;

pub trait Ui {
    fn create(ctx: &Context, create_output: impl FnMut(&Context) -> FullOutput) {}
}

pub struct Settings {
    output: FullOutput,
}

impl Ui for Settings {
    fn create(ctx: &Context, mut create_output: impl FnMut(&Context) -> FullOutput) {
            let res = egui::Window::new(WidgetText::default().strong())
                .open(&mut true)
                .vscroll(true)
                .resizable(true)
                .show(ctx, |ui| ui.label("Hello world!"));
            let full_output = create_output(ctx);

            let x = Self {
                output: full_output
            };
    }
}
