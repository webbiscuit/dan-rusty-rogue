use bevy_ecs::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
