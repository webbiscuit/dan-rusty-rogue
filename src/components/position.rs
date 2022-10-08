use bevy_ecs::prelude::*;

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn zero() -> Position {
        Position { x: 0, y: 0 }
    }
}
