use bevy_ecs::{
    schedule::{Schedule, Stage},
    world::World,
};
use bracket_terminal::prelude::{render_draw_buffer, BTerm, GameState};

use crate::resources::user_command::{self, Command, UserCommand};

pub struct State {
    world: World,
    schedule: Schedule,
}

impl Default for State {
    fn default() -> Self {
        Self::new(World::new(), Schedule::default())
    }
}

impl State {
    pub fn new(world: World, schedule: Schedule) -> Self {
        State { world, schedule }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // ctx.set_active_console(0);
        // TODO can optimise this
        ctx.cls();

        let mut user_command = self.world.get_resource_mut::<UserCommand>().unwrap();
        user_command.handle_keypress(&ctx.key);

        if let Some(command) = user_command.current_command() {
            if *command == Command::Quit {
                ctx.quit();
            }
        }

        // Run the schedule once. If your app has a "loop", you would run this once per loop
        self.schedule.run(&mut self.world);

        render_draw_buffer(ctx).expect("Render error");
    }
}
