mod console;
mod logic;
mod ui;

use anyhow::Result;

use console::Console;
use logic::{Action, Game};
use ui::{Command, Theme};

pub fn run() -> Result<()> {
    let theme = Theme {};
    let mut console = Console::new(80, 50, "Yet Another Roguelike Tutorial")?;
    let mut game = Game::new();

    loop {
        console.clear();
        ui::render_map(&mut console, theme, &game);
        ui::render_entities(&mut console, theme, &game);
        ui::render_player(&mut console, theme, &game);
        console.display()?;

        match ui::get_command(&mut console)? {
            Command::Move(dx, dy) => game
                .player_action(Action::move_by(dx, dy))
                .or_else(|_| console.alert())?,
            Command::Redraw => console.reset()?,
            Command::Abort => break,
        };
    }

    Ok(())
}
