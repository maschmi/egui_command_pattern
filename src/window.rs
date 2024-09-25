use crate::app::WindowContent;
use crate::command_handler::Command;
use egui::{Context, Id, Ui, Window};


pub(crate) fn create_window<F>(cmd_callback: &mut F, ctx: &Context, content: &WindowContent)
where
    F: FnMut(Command),
{
    Window::new("I'm a window")
        .id(Id::new(content.id)) // needed as the title is the same
        .show(ctx, |ui| {
            ui.label(content.content.to_string());
            add_close_button(cmd_callback, content, ui);
            add_noop_button(cmd_callback, ui);
        });
}

fn add_close_button<F>(cmd_callback: &mut F, content: &WindowContent, ui: &mut Ui)
where
    F: FnMut(Command),
{
    if ui.button("Close").clicked() {
        cmd_callback(Command::CloseWindow(content.id));
    }
}

fn add_noop_button<F>(cmd_callback: &mut F, ui: &mut Ui)
where
    F: FnMut(Command),
{
    if ui.button("NoOP").clicked() {
        cmd_callback(Command::NoOP);
    }
}
