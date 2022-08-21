use bracket_terminal::prelude::VirtualKeyCode;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Quit,
}

pub struct UserCommand {
    current_command: Option<Command>,
}

impl UserCommand {
    pub fn new() -> Self {
        UserCommand {
            current_command: None,
        }
    }

    pub fn handle_keypress(&mut self, key: &Option<VirtualKeyCode>) {
        if let Some(key) = key {
            match key {
                VirtualKeyCode::Up | VirtualKeyCode::W => {
                    self.current_command = Some(Command::MoveUp);
                }
                VirtualKeyCode::Down | VirtualKeyCode::S => {
                    self.current_command = Some(Command::MoveDown);
                }
                VirtualKeyCode::Left | VirtualKeyCode::A => {
                    self.current_command = Some(Command::MoveLeft);
                }
                VirtualKeyCode::Right | VirtualKeyCode::D => {
                    self.current_command = Some(Command::MoveRight);
                }
                VirtualKeyCode::Escape => {
                    self.current_command = Some(Command::Quit);
                }
                _ => self.current_command = None,
            }
        } else {
            self.current_command = None;
        }
    }

    pub fn current_command(&self) -> &Option<Command> {
        &self.current_command
    }
}
