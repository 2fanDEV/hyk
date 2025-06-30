use egui::{ClippedPrimitive, RawInput, Shape};
use egui_winit::State;
use log::debug;

use crate::core::utils::ranged::Ranged;

use super::{create_mesh_details, Meshes, Ui};

pub struct SettingsMenu {
    pub volume: Ranged<u8>,
    primitives: Vec<ClippedPrimitive>,
}

impl Ui for SettingsMenu {
    fn create(state: &mut State, raw_input: RawInput) -> Self {
        let ctx = state.egui_ctx();
        println!("TEST: {:p}", ctx);
        let mut clipped_primitives: Vec<ClippedPrimitive> = Vec::new();
        #[allow(irrefutable_let_patterns)]
        while let output = state.egui_ctx().run(raw_input.clone(), |ctx| {
            egui::Window::new("Tiesto")
                .title_bar(true)
                .vscroll(true)
                .resizable(true)
                .open(&mut true)
                .show(ctx, |ui| {
                    ui.label("Hello World");
                    if ui.button("Test").clicked() {
                        debug!("Test button");
                    }
                })
                .unwrap();
        }) {
            if !output
                .shapes
                .iter()
                .find(|pred| !pred.shape.eq(&Shape::Noop))
                .is_some()
            {
                continue;
            }
            clipped_primitives = ctx.tessellate(output.shapes, ctx.pixels_per_point());
            break;
        }

        debug!("Settings UI created");
        Self {
            volume: Ranged::new(50u8, 0, 100).unwrap(),
            primitives: clipped_primitives,
        }
    }

    fn meshes(&self) -> Vec<Meshes> {
        create_mesh_details(&self.primitives)
    }

    fn primitives(&self) -> &[ClippedPrimitive] {
        &self.primitives
    }
}
