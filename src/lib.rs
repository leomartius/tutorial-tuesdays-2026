mod console;
mod logic;
mod ui;

use anyhow::Result;

use console::Console;
use console::Event;
use logic::{Game, PlayerCommand};
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

        let event = console.read_event()?;
        match event {
            Event::Abort => break,
            Event::ClearScreen => console.reset()?,
            event => {
                let command = ui::map_play_command(event);
                match command {
                    Some(Command::Bump(dx, dy)) => game
                        .player_command(PlayerCommand::Bump(dx, dy))
                        .or_else(|_| console.alert())?,
                    Some(Command::Wait) => game
                        .player_command(PlayerCommand::Wait)
                        .or_else(|_| console.alert())?,
                    None => console.alert()?,
                }
            }
        }
    }

    Ok(())
}
