use bevy_ecs::{query::Changed, system::Query};
use bracket_terminal::prelude::{DrawBatch, Point};

use crate::components::{position::Position, render::Render};

pub fn entity_render(query: Query<(&Position, &Render, Changed<Position>)>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    query
        .iter()
        .for_each(|(position, render, needs_rendering)| {
            // TODO can optimise this?
            if needs_rendering || true {
                println!("{:?} {:?}", position, render);
                draw_batch.set(
                    Point::new(position.x, position.y),
                    render.colour,
                    render.glyph,
                );
            }
        });

    draw_batch.submit(5000).expect("Batch error");
}

// // use bracket_lib::prelude::DrawBatch;
// // use legion::{system, world::SubWorld, IntoQuery};

// // use crate::{camera::Camera, components::render::Render, point::Point};

// // #[system]
// // #[write_component(Point)]
// // #[read_component(Render)]
// pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {}
// //     let mut draw_batch = DrawBatch::new();
// //     draw_batch.target(1);

// //     let offset = Point::new(camera.left_x, camera.top_y);

// //     <(&Point, &Render)>::query()
// //         .iter(ecs)
// //         .for_each(|(pos, render)| {
// //             let point = *pos - offset;
// //             draw_batch.set(
// //                 bracket_lib::prelude::Point::new(point.x, point.y),
// //                 render.colour,
// //                 render.glyph,
// //             );
// //         });

// //     draw_batch.submit(5000).expect("Batch error");
// // }
