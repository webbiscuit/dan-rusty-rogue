use bevy_ecs::{
    schedule::{Schedule, Stage},
    world::World,
};
use bracket_terminal::prelude::*;

use crate::{
    console_consts,
    consts::*,
    resources::user_command::{Command, UserCommand},
};

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
        // TODO can optimise this
        ctx.set_active_console(console_consts::Console::World.into());
        ctx.cls();
        ctx.set_active_console(console_consts::Console::Ui.into());
        ctx.cls();

        let mut user_command = self.world.get_resource_mut::<UserCommand>().unwrap();
        user_command.handle_keypress(ctx.key);

        if let Some(command) = user_command.current_command() {
            if *command == Command::Quit {
                ctx.quit();
            }
        }

        draw_ui(ctx);

        // Run the schedule once. If your app has a "loop", you would run this once per loop
        self.schedule.run(&mut self.world);

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn draw_ui(ctx: &mut BTerm) {
    ctx.draw_box(0, 0, VIEW_WIDTH, VIEW_HEIGHT, WHITE, BLACK);

    let title_box_height: i32 = 20;
    let title_box_width: i32 = (DISPLAY_WIDTH - VIEW_WIDTH - 1) as i32;

    ctx.draw_box(
        VIEW_WIDTH,
        0,
        title_box_width,
        title_box_height,
        WHITE,
        BLACK,
    );
    ctx.print_centered_at(
        VIEW_WIDTH + (title_box_width / 2) as u32,
        title_box_height / 2,
        "Dan World",
    );

    let description_box_height: i32 = (DISPLAY_HEIGHT as i32) - title_box_height - 1;
    let description_box_width: i32 = (DISPLAY_WIDTH - VIEW_WIDTH - 1) as i32;

    ctx.draw_box(
        VIEW_WIDTH,
        title_box_height,
        description_box_width,
        description_box_height,
        WHITE,
        BLACK,
    );

    // TODO some word wrap library here
    ctx.print(
        VIEW_WIDTH + 1 as u32,
        title_box_height + 1,
        "You are in a small room",
    );
    ctx.print(VIEW_WIDTH + 1 as u32, title_box_height + 3, "...");
    ctx.print(
        VIEW_WIDTH + 1 as u32,
        title_box_height + 5,
        "It smells really funky",
    );

    // ctx.print(
    //     VIEW_WIDTH + 1 as u32,
    //     title_box_height + 1,
    //     buf, //"Welcome to Dan World!\nThings\noh now",
    // );

    // ctx.print_centered_at(
    //     VIEW_WIDTH + (description_box_width / 2) as u32,
    //     title_box_height + (description_box_height / 2),
    //     "Som text",
    // );
}
