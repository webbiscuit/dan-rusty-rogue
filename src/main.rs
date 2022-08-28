bracket_terminal::add_wasm_support!();
use bevy_ecs::prelude::*;
use bracket_terminal::prelude::*;
use components::{player::Player, position::Position, render::Render};
use consts::*;
use resources::user_command::UserCommand;
use state::State;
use systems::{entity_render::entity_render, handle_input::handle_input};

mod components;
mod console_consts;
mod consts;
mod resources;
mod state;
mod systems;

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dan Rogue World")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        // .with_tile_dimensions(32, 32)
        // .with_resource_path("resources/")
        // .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        // .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        // .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "terminal8x8.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "terminal8x8.png")
        .build()?;

    // Create a new empty World to hold our Entities and Components
    let mut world = World::new();

    world.insert_resource(UserCommand::new());

    // Spawn an entity with Position and Velocity components
    world
        .spawn()
        .insert(Position { x: 2, y: 2 })
        .insert(Render {
            colour: ColorPair::new(RED, BLACK),
            glyph: to_cp437('@'),
        })
        .insert(Player);
    let mut schedule = Schedule::default();

    // Add a Stage to our schedule. Each Stage in a schedule runs all of its systems
    // before moving on to the next Stage
    schedule.add_stage("input", SystemStage::parallel().with_system(handle_input));
    schedule.add_stage("render", SystemStage::parallel().with_system(entity_render));

    let gs: State = State::new(world, schedule);
    main_loop(context, gs)
}
