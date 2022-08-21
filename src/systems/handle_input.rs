use bevy_ecs::{
    query::With,
    system::{Query, Res},
};
use bracket_terminal::prelude::Point;

use crate::{
    components::{player::Player, position::Position},
    resources::user_command::{Command, UserCommand},
};

pub fn handle_input(resource: Res<UserCommand>, mut query: Query<&mut Position, With<Player>>) {
    // println!("{:?}", resource.current_command());
    if resource.current_command().is_some() {
        let delta = match resource.current_command().unwrap() {
            Command::MoveLeft => Point { x: -1, y: 0 },
            Command::MoveRight => Point { x: 1, y: 0 },
            Command::MoveUp => Point { x: 0, y: -1 },
            Command::MoveDown => Point { x: 0, y: 1 },
            _ => Point { x: 0, y: 0 },
        };

        println!("{:?}", delta);

        for (mut position) in &mut query {
            println!("{:?}", position);

            position.x += delta.x;
            position.y += delta.y;

            println!("{:?}", position);
        }
    }
}
