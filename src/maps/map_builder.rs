use bracket_random::prelude::RandomNumberGenerator;

use crate::components::point::Point;
use crate::maths::rect::Rect;

use super::map::{Map, Tile};

pub struct MapBuilder {
    map: Map,
    player_start: Point,
    rooms: Vec<Rect>,
}

impl MapBuilder {
    pub fn new(
        rng: &mut RandomNumberGenerator,
        map_width: u32,
        map_height: u32,
        room_count: u8,
    ) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(map_width, map_height),
            player_start: Point::zero(),
            rooms: Vec::new(),
        };
        mb.fill(Tile::Wall);
        mb.build_random_rooms(rng, map_width, map_height, room_count);
        mb.connect_rooms(rng);
        mb.player_start = mb.rooms[0].center();

        mb
    }

    fn fill(&mut self, tile: Tile) {
        self.map.fill(tile);
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn player_start(&self) -> &Point {
        &self.player_start
    }

    fn build_random_rooms(
        &mut self,
        rng: &mut RandomNumberGenerator,
        map_width: u32,
        map_height: u32,
        room_count: u8,
    ) {
        let max_room_size = 10;
        let min_room_size = 2;

        'rooms: while self.rooms.len() < room_count.into() {
            let room = Rect::with_size(
                rng.range(1, (map_width - max_room_size - 1) as i32),
                rng.range(1, (map_height - max_room_size - 1) as i32),
                rng.range(min_room_size, max_room_size as i32),
                rng.range(min_room_size, max_room_size as i32),
            );

            for r in &self.rooms {
                if r.intersect(&room) {
                    continue 'rooms;
                }
            }

            room.for_each_point(|p| {
                self.map.set_tile_at(p.x, p.y, Tile::Floor);
            });

            self.rooms.push(room);
        }
    }

    fn dig_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};

        for y in min(y1, y2)..=max(y1, y2) {
            self.map.set_tile_at(x, y, Tile::Floor);
        }
    }

    fn dig_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};

        for x in min(x1, x2)..=max(x1, x2) {
            self.map.set_tile_at(x, y, Tile::Floor);
        }
    }

    fn connect_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().y.cmp(&b.center().y));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let current = room.center();

            if rng.range(0, 2) == 1 {
                self.dig_horizontal_tunnel(prev.x, current.x, prev.y);
                self.dig_vertical_tunnel(prev.y, current.y, current.x);
            } else {
                self.dig_vertical_tunnel(prev.y, current.y, prev.x);
                self.dig_horizontal_tunnel(prev.x, current.x, current.y);
            }
        }
    }
}
