use cardiac_core::Assembler;
use eframe::{
    egui::{Button, CentralPanel, Color32, Context, Hyperlink, Label, Layout, Vec2, Visuals},
    epi::App,
    run_native, NativeOptions,
};

const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const RED: Color32 = Color32::from_rgb(255, 0, 0);

struct CardiacConfig {
    dark_mode: bool,
}

struct Cardiac {
    assembler: Assembler,
    input_text: String,
    config: CardiacConfig,
}

impl Cardiac {
    fn new() -> Self {
        let config: CardiacConfig = CardiacConfig { dark_mode: true };
        Self {
            assembler: Assembler::new(),
            input_text: String::new(),
            config: config,
        }
    }

    fn render_header(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(), |ui| {
                ui.button("tools");
                ui.collapsing("Instructions", |ui| {
                    ui.label("Code  Mnemonic  Instruction");
                    ui.label("0          INP                 Input");
                    ui.label("1          CLA                Clear and add");
                    ui.label("2          ADD               Add");
                    ui.label("3          TAC                Test accumulator contents");
                    ui.label("4          SFT                 Shift");
                    ui.label("5          OUT               Output");
                    ui.label("6          STO                Store");
                    ui.label("7          SUB                Subtract");
                    ui.label("8          JMP               Jump");
                    ui.label("9          HRS                Halt and reset");
                });
            });

            ui.with_layout(Layout::right_to_left(), |ui| {
                let theme_btn = ui.add(Button::new({
                    if self.config.dark_mode {
                        "🌞"
                    } else {
                        "🌙"
                    }
                }));
                if theme_btn.clicked() {
                    self.config.dark_mode = !self.config.dark_mode;
                }
            });
        });
    }

    fn render_memory(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.columns(10, |cols| {
                for i in 0..10 {
                    for j in 0..10 {
                        let idx = i * 10 + j;
                        let cell = self.assembler.get_memory_cell(idx);
                        let drag_value = eframe::egui::DragValue::new(cell);
                        cols[i].add(drag_value);

                        let label = Label::new(format!("{}", idx)).text_color(
                            if *self.assembler.get_target() == idx as u32 {
                                if self.config.dark_mode {
                                    CYAN
                                } else {
                                    RED
                                }
                            } else {
                                Color32::LIGHT_GRAY
                            },
                        );
                        cols[i].label(label);
                    }
                }
            });
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
        // Buttons
        ui.horizontal_wrapped(|ui| {
            if ui.button("step").clicked() {
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

    fn render_body(&mut self, ui: &mut eframe::egui::Ui) {
        ui.columns(4, |cols| {
            self.render_controls(&mut cols[0]);
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

    fn render_footer(&self, ui: &mut eframe::egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.add(Hyperlink::from_label_and_url(
                "Made with egui",
                "https://github.com/emilk/egui",
            ));
            ui.add(Hyperlink::from_label_and_url(
                "Xukay101/CardiacVisualizer",
                "https://github.com/Xukay101/CardiacVisualizer",
            ));
            ui.add_space(10.);
        });
    }
}

impl App for Cardiac {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        // Update logic
        if self.assembler.check_run() {
            self.assembler.next_step()
        }

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        // Render interface
        CentralPanel::default().show(ctx, |ui| {
            // Doc in https://www.egui.rs/
            self.render_header(ui);
            ui.separator();
            self.render_memory(ui);
            ui.separator();
            self.render_body(ui);
            ui.separator();
            self.render_footer(ui)
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
    win_option.resizable = false;

    run_native(Box::new(app), win_option)
}
