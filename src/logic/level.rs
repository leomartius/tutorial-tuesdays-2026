use super::{Tile, Vec2};

pub struct Level {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl Level {
    pub fn new(width: i32, height: i32) -> Self {
        debug_assert!(width > 0 && height > 0);
        Self {
            width,
            height,
            tiles: vec![Tile::Floor; (width as usize) * (height as usize)],
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
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

    pub fn is_walkable(&self, pos: Vec2) -> bool {
        self.get_tile(pos) == Tile::Floor
    }
}
