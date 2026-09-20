mod level;

use level::Level;

pub struct Game {
    player: (i32, i32),
    level: Level,
}

impl Game {
    pub fn new() -> Self {
        let mut level = Level::new(80, 45);
        for x in 30..33 {
            level.set_tile(x, 22, Tile::Wall);
        }
        Self {
            player: (40, 25),
            level,
        }
    }

    pub fn player(&self) -> (i32, i32) {
        self.player
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) -> Result<(), ()> {
        let x = self.player.0 + dx;
        let y = self.player.1 + dy;
        if self.level.in_bounds(x, y) && self.level.is_walkable(x, y) {
            self.player = (x, y);
            return Ok(());
        }
        Err(())
    }

    pub fn level(&self) -> &Level {
        &self.level
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
}
