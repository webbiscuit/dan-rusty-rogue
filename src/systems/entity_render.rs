use bevy_ecs::{query::Changed, system::Query};
use bracket_terminal::prelude::{DrawBatch, Point};

use crate::{
    components::{position::Position, render::Render},
    console_consts,
};

pub fn entity_render(query: Query<(&Position, &Render, Changed<Position>)>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(console_consts::Console::World.into());

    query
        .iter()
        .for_each(|(position, render, _needs_rendering)| {
            // TODO can optimise this?
            // if needs_rendering || true {
            // println!("{:?} {:?}", position, render);
            draw_batch.set(
                Point::new(position.x, position.y),
                render.colour,
                render.glyph,
            );
        });

    draw_batch
        .submit(console_consts::Layer::Entities.into())
        .expect("Batch error");
}
