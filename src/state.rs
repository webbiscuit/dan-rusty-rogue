use bevy_ecs::{schedule::Schedule, world::World};
use bracket_terminal::prelude::*;

use crate::{
    console_consts,
    consts::{DISPLAY_HEIGHT, DISPLAY_WIDTH, VIEW_HEIGHT, VIEW_WIDTH},
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
                log::info!("Quitting game");
                ctx.quit();
                return;
            }
        }

        draw_ui(ctx);
        draw_overlays(ctx);

        // Run the schedule once. If your app has a "loop", you would run this once per loop
        self.schedule.run(&mut self.world);

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn draw_overlays(ctx: &mut BTerm) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(console_consts::Console::Overlays.into());
    draw_batch.cls();
    draw_batch.draw_double_box(
        Rect::with_size(39, 0, 20, 3),
        ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
    );
    draw_batch.print_color(
        Point::new(40, 1),
        &format!("Blah: {}", ctx.fps),
        ColorPair::new(RGB::named(YELLOW), RGB::named(BLACK)),
    );

    draw_batch.submit(0).expect("Batch error");
}

fn draw_ui(ctx: &mut BTerm) {
    let title_box_height: i32 = 20;
    let title_box_width: i32 = (DISPLAY_WIDTH - VIEW_WIDTH - 1) as i32;
    let description_box_height: i32 = (DISPLAY_HEIGHT as i32) - title_box_height - 1;
    let description_box_width: i32 = (DISPLAY_WIDTH - VIEW_WIDTH - 1) as i32;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(console_consts::Console::Ui.into());
    draw_batch.cls();

    draw_batch.draw_box(
        Rect::with_size(0, 0, VIEW_WIDTH, VIEW_HEIGHT),
        ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
    );

    draw_batch.draw_box(
        Rect::with_size(VIEW_WIDTH as i32, 0, title_box_width, title_box_height),
        ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
    );

    draw_batch.print_color_centered_at(
        Point::new(
            VIEW_WIDTH as i32 + (title_box_width / 2) as i32,
            title_box_height / 2,
        ),
        "Dan World",
        ColorPair::new(RGB::named(CYAN), RGB::named(BLACK)),
    );

    draw_batch.draw_box(
        Rect::with_size(
            VIEW_WIDTH as i32,
            title_box_height,
            description_box_width,
            description_box_height,
        ),
        ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
    );

    let mut buf = TextBuilder::empty();
    buf.ln()
        .line_wrap(
            "You are obviously a wizard. You have been summoned to the Land of Dan to help people and be generally awesome.",
        )
        .reset();

    let mut block = TextBlock::new(
        (VIEW_WIDTH + 1 + 1) as i32,
        title_box_height + 1,
        description_box_width - 1,
        description_box_height - 1,
    );

    block.print(&buf).expect("Text too long");
    block.render_to_draw_batch(&mut draw_batch);
    draw_batch.submit(0).expect("Batch error");
    render_draw_buffer(ctx).expect("Render error");
}
