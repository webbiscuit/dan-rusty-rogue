#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

#[derive(Clone)]
pub struct Map {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        Map {
            tiles: vec![Tile::Floor; (width * height).try_into().unwrap()],
            width,
            height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn tile_at(&self, x: i32, y: i32) -> Tile {
        self.tiles[to_index(x, y, self.width as i32)]
    }

    pub fn set_tile_at(&mut self, x: i32, y: i32, tile: Tile) {
        self.tiles[to_index(x, y, self.width as i32)] = tile;
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        (0..self.width as i32).contains(&x) && (0..self.height as i32).contains(&y)
    }

    pub fn can_enter_tile(&self, x: i32, y: i32) -> bool {
        self.in_bounds(x, y) && self.tile_at(x, y) == Tile::Floor
    }

    pub fn fill(&mut self, tile: Tile) {
        self.tiles.iter_mut().for_each(|t| *t = tile);
    }
}

pub fn to_index(x: i32, y: i32, width: i32) -> usize {
    (x + y * width) as usize
}
