use bevy_ecs::system::Res;

use crate::resources::user_command::UserCommand;

pub fn handle_input(resource: Res<UserCommand>) {
    if resource.current_command().is_some() {
        println!("{:?}", resource.current_command());
    }
}
