use bevy_ecs::{
    schedule::{Schedule, Stage},
    world::World,
};
use bracket_terminal::prelude::{render_draw_buffer, BTerm, GameState};

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
        // ctx.cls();

        // Run the schedule once. If your app has a "loop", you would run this once per loop
        self.schedule.run(&mut self.world);

        render_draw_buffer(ctx).expect("Render error");
    }
}
