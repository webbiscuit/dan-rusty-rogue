use bevy_ecs::system::Resource;
use bracket_terminal::prelude::VirtualKeyCode;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Command {
    Quit,
    TryMove(Direction),
}

#[derive(Resource)]
pub struct UserCommand {
    current_command: Option<Command>,
}

impl UserCommand {
    pub fn new() -> Self {
        UserCommand {
            current_command: None,
        }
    }

    #[cfg(test)]
    pub fn set_command(&mut self, command: Command) {
        self.current_command = Some(command);
    }

    pub fn handle_keypress(&mut self, key: Option<VirtualKeyCode>) {
        if let Some(key) = key {
            log::info!("key: {:?}", key);
            match key {
                VirtualKeyCode::Up | VirtualKeyCode::W => {
                    self.current_command = Some(Command::TryMove(Direction::Up));
                }
                VirtualKeyCode::Down | VirtualKeyCode::S => {
                    self.current_command = Some(Command::TryMove(Direction::Down));
                }
                VirtualKeyCode::Left | VirtualKeyCode::A => {
                    self.current_command = Some(Command::TryMove(Direction::Left));
                }
                VirtualKeyCode::Right | VirtualKeyCode::D => {
                    self.current_command = Some(Command::TryMove(Direction::Right));
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
