use bevy_ecs::prelude::*;
use bracket_terminal::{FontCharType, prelude::ColorPair};

#[derive(Component, Debug)]
pub struct Render {
    pub colour: ColorPair,
    pub glyph: FontCharType,
}
