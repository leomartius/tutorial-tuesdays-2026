mod console;
mod logic;
mod ui;

use anyhow::Result;

use console::Console;
use logic::Game;
use ui::Command;

pub fn run() -> Result<()> {
    let mut console = Console::new(80, 50, "Yet Another Roguelike Tutorial")?;
    let mut game = Game::new();

    loop {
        console.clear();
        ui::render_map(&mut console, &game);
        ui::render_entities(&mut console, &game);
        ui::render_player(&mut console, &game);
        console.display()?;

        match ui::get_command(&mut console)? {
            Command::Move(dx, dy) => game.move_player(dx, dy).or_else(|_| console.alert())?,
            Command::Redraw => console.reset()?,
            Command::Abort => break,
        };
    }

    Ok(())
}
