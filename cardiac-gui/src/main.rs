use cardiac_core::Assembler;
use eframe::{
    egui::{CentralPanel, Color32, Label, Vec2},
    epi::App,
    run_native, NativeOptions,
};

struct Cardiac {
    assembler: Assembler,
    input_text: String,
}

impl Cardiac {
    fn new() -> Self {
        Self {
            assembler: Assembler::new(),
            input_text: String::new(),
        }
    }

    fn render_header(&self, ui: &mut eframe::egui::Ui) {
        ui.label("Hello world");
    }

    fn render_memory(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            for i in 0..10 {
                ui.vertical(|ui| {
                    for j in 0..10 {
                        let idx = i * 10 + j;
                        let cell = self.assembler.get_memory_cell(idx);
                        let drag_value = eframe::egui::DragValue::new(cell);
                        ui.add(drag_value);

                        let label = Label::new(format!("{}", idx)).text_color(
                            if *self.assembler.get_target() == idx as u32 {
                                Color32::LIGHT_BLUE
                            } else {
                                Color32::LIGHT_GRAY
                            },
                        );
                        ui.label(label);
                    }
                });
            }
        });
    }

    fn render_controls(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label(format!("flag: {}", self.assembler.get_flag()));
        ui.label(format!("accumulator: {}", self.assembler.get_accumulator()));
        ui.label(format!("step: {}", self.assembler.get_step()));
        ui.horizontal(|ui| {
            let target = self.assembler.get_target();
            let drag_value = eframe::egui::DragValue::new(target);
            ui.label("target");
            ui.add(drag_value);
        });
        // buttons
        ui.horizontal(|ui| {
            if ui.button("next step").clicked() {
                self.assembler.next_step();
            } else if ui.button("run").clicked() {
                self.assembler.set_run(!self.assembler.check_run());
            } else if ui.button("reset").clicked() {
                self.assembler.reset();
            } else if ui.button("clear memory").clicked() {
                self.assembler.clear_memory();
            }
        });
    }

    fn render_input(&mut self, ui: &mut eframe::egui::Ui) {
        let input_card = self.assembler.get_input_card();
        ui.label(format!("input: {:?}", input_card));
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.input_text);
            if ui.button("add").clicked() {
                let value = match self.input_text.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => 0,
                };

                if value != 0 && (value >= -999 && value <= 999) {
                    self.assembler.add_input(value);
                }
                self.input_text.clear();
            }
        });
    }

    fn render_output(&mut self, ui: &mut eframe::egui::Ui) {
        let output_card = self.assembler.get_output_card();
        ui.label(format!("output: {:?}", output_card));
    }

}

impl App for Cardiac {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        // Update logic
        if self.assembler.check_run() {
            self.assembler.next_step()
        }

        // Render interface
        CentralPanel::default().show(ctx, |ui| {
            // Doc in https://www.egui.rs/
            self.render_header(ui);
            ui.separator();
            self.render_memory(ui);
            ui.separator();
            self.render_controls(ui);
            ui.separator();
            self.render_input(ui);
            ui.separator();
            self.render_output(ui);
        });
    }

    fn name(&self) -> &str {
        "Cardiac"
    }
}

fn main() {
    let app = Cardiac::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 760.));

    run_native(Box::new(app), win_option)
}
