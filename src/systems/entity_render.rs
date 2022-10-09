use bevy_ecs::{
    query::Changed,
    system::{Query, Res},
};
use bracket_terminal::prelude::{
    to_cp437, ColorPair, DrawBatch, Point as BracketPoint, BLACK, GREY,
};

use crate::{
    components::{point::Point, render::Render},
    console_consts,
    consts::{MAP_VIEW_OFFSET_X, MAP_VIEW_OFFSET_Y},
    maps::map::{Map, Tile},
};

pub fn entity_render(map: Res<Map>, query: Query<(&Point, &Render, Changed<Point>)>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(console_consts::Console::World.into());

    let offset = BracketPoint::new(MAP_VIEW_OFFSET_X, MAP_VIEW_OFFSET_Y);

    for x in 0..map.width() {
        for y in 0..map.height() {
            let tile = map.tile_at(x as i32, y as i32);
            let glyph = match tile {
                Tile::Floor => to_cp437(' '),
                Tile::Wall => to_cp437('#'),
            };
            draw_batch.set(
                BracketPoint::new(x, y) + offset,
                ColorPair::new(GREY, BLACK),
                glyph,
            );
        }
    }

    query
        .iter()
        .for_each(|(position, render, _needs_rendering)| {
            // TODO can optimise this?
            // if needs_rendering || true {
            // println!("{:?} {:?}", position, render);
            draw_batch.set(
                BracketPoint::new(position.x, position.y) + offset,
                render.colour,
                render.glyph,
            );
        });

    draw_batch
        .submit(console_consts::Layer::Entities.into())
        .expect("Batch error");
}
