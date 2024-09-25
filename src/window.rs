use egui::{Context, Id, Ui, Window};
use crate::app::WindowContent;
use crate::command_handler::{Command, CommandHandler};
use crate::CommandPatternApp;

impl CommandPatternApp {
    pub(crate) fn create_window(&mut self, ctx: &Context, content: &WindowContent) {
        Window::new("I'm a window")
            .id(Id::new(content.id)) // needed as the title is the same
            .show(ctx, |ui| {
                ui.label(content.content.to_string());
                self.add_close_button(content, ui);
                self.add_noop_button(ui);
            });
    }

    fn add_close_button(&mut self, content: &WindowContent, ui: &mut Ui) {
        if ui.button("Close").clicked() {
            self.handle_command(Command::CloseWindow(content.id));
        }
    }

    fn add_noop_button(&mut self, ui: &mut Ui) {
        if ui.button("NoOP").clicked() {
            self.handle_command(Command::NoOP);
        }
    }
}
