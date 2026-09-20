pub struct Game {
    width: i32,
    height: i32,
    player: (i32, i32),
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            player: (width / 2, height / 2),
        }
    }

    pub fn player(&self) -> (i32, i32) {
        self.player
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) -> Result<(), ()> {
        let x = self.player.0 + dx;
        let y = self.player.1 + dy;
        if 0 <= x && x < self.width && 0 <= y && y < self.height {
            self.player = (x, y);
            return Ok(());
        }
        Err(())
    }
}
