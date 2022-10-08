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
    if resource.current_command().is_some() {
        let delta = match resource.current_command().unwrap() {
            Command::MoveLeft => Point { x: -1, y: 0 },
            Command::MoveRight => Point { x: 1, y: 0 },
            Command::MoveUp => Point { x: 0, y: -1 },
            Command::MoveDown => Point { x: 0, y: 1 },
            _ => Point { x: 0, y: 0 },
        };

        for mut position in &mut query {
            log::info!("start pos: {:?}", position.clone());

            position.x += delta.x;
            position.y += delta.y;

            log::info!("end pos: {:?}", position.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::*;

    #[test]
    fn handles_input_correctly() {
        let mut world = World::new();
        let mut schedule = Schedule::default();

        let mut user_command = UserCommand::new();
        user_command.set_command(Command::MoveLeft);
        world.insert_resource(user_command);
        let player_id = world
            .spawn()
            .insert(Player)
            .insert(Position { x: 2, y: 2 })
            .id();

        schedule.add_stage("input", SystemStage::parallel().with_system(handle_input));

        schedule.run(&mut world);

        let position = world.get::<Position>(player_id).unwrap();
        assert_eq!(position, &Position { x: 1, y: 2 });
    }
}
