// use bracket_lib::prelude::RandomNumberGenerator;

use crate::components::position::Position;

use super::map::{Map, Tile};

pub struct MapBuilder {
    map: Map,
    player_start: Position,
}

impl MapBuilder {
    pub fn new() -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            player_start: Position::zero(),
        };
        mb.fill(Tile::Wall);
        mb.player_start = Position::new(10, 10);

        mb
    }

    fn fill(&mut self, tile: Tile) {
        self.map.fill(tile);
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn player_start(&self) -> &Position {
        &self.player_start
    }
}
