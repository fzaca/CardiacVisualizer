use eframe::{egui::CentralPanel, epi::App, run_native, NativeOptions};

struct Cardiac;

impl App for Cardiac {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello world");
        });
    }

    fn name(&self) -> &str {
        "Cardiac"
    }
}

fn main() {
    let app = Cardiac;
    let win_option = NativeOptions::default();

    run_native(Box::new(app), win_option)
}
