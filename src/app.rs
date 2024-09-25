use egui::{Context, Ui};

pub struct CommandPatternApp {
    // Example stuff:
    label: String,
    correct_answer: Option<bool>,
    value: f32,
}

impl Default for CommandPatternApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            correct_answer: None,
            value: 2.7,
        }
    }
}

impl CommandPatternApp {
    /// Called once before the first frame.
    pub fn new() -> Self {
        Default::default()
    }

    fn create_top_menu(ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }

    fn create_central_panel(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Command pattern example");

            ui.horizontal(|ui| {
                ui.label("What is 5 + 5?");
                ui.text_edit_singleline(&mut self.label);
                if ui.button("Check").clicked() {
                    self.correct_answer = Some(10 == self.label.parse::<i32>().unwrap_or(0));
                };
            });
            if let Some(answer) = self.correct_answer {
                let label_text = if answer {
                    "Your answer was correct."
                } else {
                    "Your answer was wrong."
                };
                ui.label(label_text);
            } else {
                ui.label("");
            }

            ui.add_space(10.0);

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            Self::add_footer(ui);
        });
    }

    fn add_footer(ui: &mut Ui) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
    }
}

impl eframe::App for CommandPatternApp {

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Self::create_top_menu(ctx);

        self.create_central_panel(ctx);
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
