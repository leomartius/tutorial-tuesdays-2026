mod console;
mod logic;
mod ui;

use anyhow::Result;

use console::{Color, Console};
use logic::Game;
use ui::Command;

pub fn run() -> Result<()> {
    let mut game = Game::new(80, 50);
    let mut console = Console::new(80, 50, "Yet Another Roguelike Tutorial")?;

    loop {
        console.clear();
        let (x, y) = game.player();
        console.print_char(x, y, '@', Color::Default);
        console.show_cursor(x, y);
        console.display()?;

        match ui::get_command(&mut console)? {
            Command::Move(dx, dy) => game.move_player(dx, dy).or_else(|_| console.alert())?,
            Command::Redraw => console.reset()?,
            Command::Abort => break,
        };
    }

    Ok(())
}
