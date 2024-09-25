use crate::command_handler::{Command, CommandHandler};
use crate::window::create_window;
use egui::{Context, Id, Ui, Window};

pub struct CommandPatternApp {
    // Example stuff:
    pub(crate) label: String,
    pub(crate) correct_answer: Option<bool>,
    pub(crate) value: f32,
    pub(crate) window_id: usize,
    pub(crate) open_windows: Vec<usize>, // this is not optimal, but better understandable then a map or a btree
    pub(crate) cmd_to_run: Option<Command>
}

#[derive(Clone)]
pub(crate) struct WindowContent {
    pub(crate) id: usize,
    pub(crate) content: String,
}

impl Default for CommandPatternApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            correct_answer: None,
            value: 2.7,
            open_windows: vec![],
            window_id: 0,
            cmd_to_run: None,
        }
    }
}


impl WindowContent {
    fn create(id: usize) -> Self {
        WindowContent {
            id,
            content: format!("I'm a window and my id is {}", id),
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
                    let value = self.label.clone();
                    self.cmd_to_run = Some(Command::VerifyAnswer(value));
                };
            });

            self.display_check_result(ui);

            ui.add_space(10.0);

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.cmd_to_run = Some(Command::IncrementByButton);
            }

            ui.separator();

            if ui.button("Add window").clicked() {
                self.cmd_to_run = Some(Command::CreateNewWindow(self.window_id));
                self.window_id += 1;
            }

            self.draw_windows(ctx);

            self.add_window_counter(ui);

            Self::add_footer(ui);
        });
    }

    fn add_window_counter(&mut self, ui: &mut Ui) {
       ui.label(format!("{} windows open", self.open_windows.len()));
    }

    fn display_check_result(&mut self, ui: &mut Ui) {
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
    }

    fn draw_windows(&mut self, ctx: &Context) {
        let windows_to_draw = self.open_windows.clone();
        let mut callback = |cmd: Command| self.cmd_to_run = Some(cmd);
        windows_to_draw.iter().for_each(|id| {
            let content = WindowContent::create(id.clone());
            create_window(
                &mut callback,
                ctx,
                &content,
            );
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
        if let Some(cmd) = &self.cmd_to_run {
            self.handle_command(cmd.clone())
        }
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
