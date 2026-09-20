use super::Tile;

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

    fn index(&self, x: i32, y: i32) -> usize {
        debug_assert!(self.in_bounds(x, y));
        (y as usize) * (self.width as usize) + (x as usize)
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        0 <= x && x < self.width && 0 <= y && y < self.height
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        debug_assert!(self.in_bounds(x, y));
        let index = self.index(x, y);
        self.tiles[index]
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        debug_assert!(self.in_bounds(x, y));
        let index = self.index(x, y);
        self.tiles[index] = tile;
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        debug_assert!(self.in_bounds(x, y));
        self.get_tile(x, y) == Tile::Floor
    }
}
