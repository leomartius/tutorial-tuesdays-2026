use std::cmp::{max, min};

use super::{Tile, Vec2};

pub struct Level {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
    entry: Vec2,
}

impl Level {
    pub fn new(width: i32, height: i32, entry: Vec2) -> Self {
        debug_assert!(width > 0 && height > 0);
        debug_assert!(0 <= entry.x && entry.x < width && 0 <= entry.y && entry.y < height);
        Self {
            width,
            height,
            tiles: vec![Tile::Wall; (width as usize) * (height as usize)],
            entry,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn entry(&self) -> Vec2 {
        self.entry
    }

    fn index(&self, pos: Vec2) -> usize {
        debug_assert!(self.in_bounds(pos));
        (pos.y as usize) * (self.width as usize) + (pos.x as usize)
    }

    pub fn in_bounds(&self, pos: Vec2) -> bool {
        0 <= pos.x && pos.x < self.width && 0 <= pos.y && pos.y < self.height
    }

    pub fn get_tile(&self, pos: Vec2) -> Tile {
        let index = self.index(pos);
        self.tiles[index]
    }

    pub fn set_tile(&mut self, pos: Vec2, tile: Tile) {
        let index = self.index(pos);
        self.tiles[index] = tile;
    }

    pub fn fill_rect(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, value: Tile) {
        debug_assert!(0 <= x0 && x0 < self.width && 0 <= y0 && y0 < self.height);
        debug_assert!(0 <= x1 && x1 < self.width && 0 <= y1 && y1 < self.height);
        let (xmin, xmax) = (min(x0, x1), max(x0, x1));
        let (ymin, ymax) = (min(y0, y1), max(y0, y1));
        for y in ymin..=ymax {
            let row = (y as usize) * (self.width as usize);
            self.tiles[row + (xmin as usize)..=row + (xmax as usize)].fill(value);
        }
    }

    pub fn is_walkable(&self, pos: Vec2) -> bool {
        self.get_tile(pos) == Tile::Floor
    }
}
