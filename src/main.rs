bracket_terminal::add_wasm_support!();
use bevy_ecs::prelude::*;
use bracket_terminal::prelude::*;
use components::{position::Position, render::Render};
use state::State;
use systems::entity_render::entity_render;

mod components;
mod state;
mod systems;

const DISPLAY_WIDTH: u32 = 80;
const DISPLAY_HEIGHT: u32 = 50;
const MAP_WIDTH: u32 = 80;
const MAP_HEIGHT: u32 = 80;

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
        // .with_simple_console_no_bg(MAP_WIDTH * 2, MAP_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    // Create a new empty World to hold our Entities and Components
    let mut world = World::new();

    // Spawn an entity with Position and Velocity components
    world
        .spawn()
        .insert(Position { x: 2, y: 2 })
        .insert(Render {
            colour: ColorPair::new(RED, BLACK),
            glyph: to_cp437('@'),
        });
    let mut schedule = Schedule::default();

    // Add a Stage to our schedule. Each Stage in a schedule runs all of its systems
    // before moving on to the next Stage
    schedule.add_stage("render", SystemStage::parallel().with_system(entity_render));

    let gs: State = State::new(world, schedule);
    main_loop(context, gs)
}
