use bevy_ecs::{
    query::With,
    system::{Query, Res},
};

use crate::{
    components::{player::Player, point::Point},
    maps::map::Map,
    resources::user_command::{Command, Direction, UserCommand},
};

pub fn handle_player_commands(
    command: Res<UserCommand>,
    map: Res<Map>,
    mut query: Query<&mut Point, With<Player>>,
) {
    if command.current_command().is_some() {
        let delta = match command.current_command().unwrap() {
            Command::TryMove(direction) => match direction {
                Direction::Up => Point::new(0, -1),
                Direction::Down => Point::new(0, 1),
                Direction::Left => Point::new(-1, 0),
                Direction::Right => Point::new(1, 0),
            },
            _ => Point { x: 0, y: 0 },
        };

        for mut position in &mut query {
            log::info!("start pos: {:?}", position);

            let new_position = position.to_owned() + delta;
            if map.can_enter_tile(new_position.x, new_position.y) {
                *position = new_position;
            }

            log::info!("end pos: {:?}", position);
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
        user_command.set_command(Command::TryMove(Direction::Left));
        let map = Map::new(10, 10);
        world.insert_resource(user_command);
        world.insert_resource(map);
        let player_id = world.spawn((Player, Point { x: 2, y: 2 })).id();

        schedule.add_system(handle_player_commands);

        schedule.run(&mut world);

        let position = world.get::<Point>(player_id).unwrap();
        assert_eq!(position, &Point { x: 1, y: 2 });
    }

    #[test]
    fn handles_map_bounds() {
        let mut world = World::new();
        let mut schedule = Schedule::default();

        let mut user_command = UserCommand::new();
        user_command.set_command(Command::TryMove(Direction::Left));
        let map = Map::new(10, 10);
        world.insert_resource(user_command);
        world.insert_resource(map);
        let player_id = world.spawn((Player, Point { x: 0, y: 0 })).id();

        schedule.add_system(handle_player_commands);

        schedule.run(&mut world);

        let position = world.get::<Point>(player_id).unwrap();
        assert_eq!(position, &Point { x: 0, y: 0 });
    }
}
