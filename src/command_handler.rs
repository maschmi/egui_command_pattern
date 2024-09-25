use crate::app::WindowContent;
use crate::CommandPatternApp;

pub(crate) enum Command {
    VerifyAnswer(String),
    IncrementByButton,
    CreateNewWindow(WindowContent),
    CloseWindow(usize),
}

pub(crate) trait CommandHandler {
    fn handle_command(&mut self, command: Command);
}

impl CommandHandler for CommandPatternApp {
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::VerifyAnswer(answer) => {
                dbg!("Handle verify answer command");
                self.correct_answer = Some(10 == answer.parse::<i32>().unwrap_or(0));
            }
            Command::IncrementByButton => {
                dbg!("Handle increment_by_button command");
                self.value += 1.0;
            }
            Command::CreateNewWindow(content) => {
                dbg!("Handle create new window command");
                self.windows.push(content.clone());
            }
            Command::CloseWindow(id) => {
                dbg!("Handle close window command");
                self.windows.retain(|w| &w.id != &id);
            }
        }
    }
}
