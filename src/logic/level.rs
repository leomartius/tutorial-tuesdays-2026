use std::cmp::{max, min};

use super::fov::compute_fov;
use super::{Pos, Tile};

pub struct Level {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
    visible: Vec<bool>,
    explored: Vec<bool>,
}

impl Level {
    pub fn new(width: i32, height: i32) -> Self {
        debug_assert!(width > 0 && height > 0);
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            tiles: vec![Tile::Wall; size],
            visible: vec![false; size],
            explored: vec![false; size],
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    fn index(&self, pos: Pos) -> usize {
        debug_assert!(self.in_bounds(pos));
        (pos.y as usize) * (self.width as usize) + (pos.x as usize)
    }

    pub fn in_bounds(&self, pos: Pos) -> bool {
        0 <= pos.x && pos.x < self.width && 0 <= pos.y && pos.y < self.height
    }

    pub fn get_tile(&self, pos: Pos) -> Tile {
        let index = self.index(pos);
        self.tiles[index]
    }

    pub fn set_tile(&mut self, pos: Pos, tile: Tile) {
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

    pub fn is_walkable(&self, pos: Pos) -> bool {
        self.get_tile(pos) == Tile::Floor
    }

    pub fn is_visible(&self, pos: Pos) -> bool {
        let index = self.index(pos);
        self.visible[index]
    }

    pub fn is_explored(&self, pos: Pos) -> bool {
        let index = self.index(pos);
        self.explored[index]
    }

    pub fn update_vision(&mut self, pov: Pos) {
        let radius = 8;
        self.visible.fill(false);
        let is_transparent =
            |x, y| self.tiles[y as usize * self.width as usize + x as usize] != Tile::Wall;
        let set_visible = |x, y| {
            self.visible[y as usize * self.width as usize + x as usize] = true;
            self.explored[y as usize * self.width as usize + x as usize] = true
        };
        compute_fov(
            (self.width, self.height),
            is_transparent,
            set_visible,
            (pov.x, pov.y),
            radius,
        );
    }
}
