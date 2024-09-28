use crate::app::WindowContent;
use crate::CommandPatternApp;

#[derive(Clone)]
pub(crate) enum Command {
    VerifyAnswer(String),
    IncrementByButton,
    CreateNewWindow(usize),
    CloseWindow(usize),
    NoOP,
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
                self.open_windows.push(content);
            }
            Command::CloseWindow(id) => {
                dbg!("Handle close window command");
                self.open_windows.retain(|w| w != &id);
            }
            Command::NoOP => {
                dbg!("Handle noop command");
            }
        }
        self.cmd_to_run = None;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_answer_command_ok() {
        let mut sut = CommandPatternApp::new();
        assert_eq!(sut.correct_answer, None); // initial state
        let cmd = Command::VerifyAnswer("10".to_string());

        sut.handle_command(cmd);

        assert_eq!(sut.correct_answer, Some(true));
    }

    #[test]
    fn test_verify_answer_command_not_ok() {
        let mut sut = CommandPatternApp::new();
        assert_eq!(sut.correct_answer, None); // initial state
        let cmd = Command::VerifyAnswer("ten".to_string());

        sut.handle_command(cmd);

        assert_eq!(sut.correct_answer, Some(false));
    }

}