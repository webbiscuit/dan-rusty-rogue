use crate::consts::{MAP_HEIGHT, MAP_WIDTH};

const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

#[derive(Clone)]
pub struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            tiles: vec![Tile::Floor; NUM_TILES],
        }
    }

    pub fn width(&self) -> u32 {
        MAP_WIDTH
    }

    pub fn height(&self) -> u32 {
        MAP_HEIGHT
    }

    pub fn tile_at(&self, x: i32, y: i32) -> Tile {
        self.tiles[to_index(x, y)]
    }

    pub fn set_tile_at(&mut self, x: i32, y: i32, tile: Tile) {
        self.tiles[to_index(x, y)] = tile;
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        (0..MAP_WIDTH as i32).contains(&x) && (0..MAP_HEIGHT as i32).contains(&y)
    }

    pub fn can_enter_tile(&self, x: i32, y: i32) -> bool {
        self.in_bounds(x, y) && self.tile_at(x, y) == Tile::Floor
    }

    pub fn fill(&mut self, tile: Tile) {
        self.tiles.iter_mut().for_each(|t| *t = tile);
    }
}

pub fn to_index(x: i32, y: i32) -> usize {
    (x + y * MAP_WIDTH as i32) as usize
}
