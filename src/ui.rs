use anyhow::Result;

use crate::console::{Color, Console, Event, Key};
use crate::logic::{Game, Tile};

pub enum Command {
    Move(i32, i32),
    Abort,
    Redraw,
}

pub fn get_command(console: &mut Console) -> Result<Command> {
    loop {
        let event = console.read_event()?;
        let command = match event {
            Event::Abort => Command::Abort,
            Event::ClearScreen => Command::Redraw,
            Event::KeyChar('y') => Command::Move(-1, -1),
            Event::KeyChar('k') => Command::Move(0, -1),
            Event::KeyChar('u') => Command::Move(1, -1),
            Event::KeyChar('h') => Command::Move(-1, 0),
            Event::KeyChar('.') => Command::Move(0, 0),
            Event::KeyChar('l') => Command::Move(1, 0),
            Event::KeyChar('b') => Command::Move(-1, 1),
            Event::KeyChar('j') => Command::Move(0, 1),
            Event::KeyChar('n') => Command::Move(1, 1),
            Event::KeySpecial(Key::Home) => Command::Move(-1, -1),
            Event::KeySpecial(Key::Up) => Command::Move(0, -1),
            Event::KeySpecial(Key::PgUp) => Command::Move(1, -1),
            Event::KeySpecial(Key::Left) => Command::Move(-1, 0),
            Event::KeySpecial(Key::Right) => Command::Move(1, 0),
            Event::KeySpecial(Key::End) => Command::Move(-1, 1),
            Event::KeySpecial(Key::Down) => Command::Move(0, 1),
            Event::KeySpecial(Key::PgDn) => Command::Move(1, 1),
            _ => {
                console.alert()?;
                continue;
            }
        };
        return Ok(command);
    }
}

const WALL_BG: Color = Color::rgb(0, 0, 100);
const FLOOR_BG: Color = Color::rgb(50, 50, 150);
const PLAYER_FG: Color = Color::rgb(255, 255, 255);

pub fn render_map(console: &mut Console, game: &Game) {
    for y in 0..game.level().height() {
        for x in 0..game.level().width() {
            let bg = match game.level().get_tile(x, y) {
                Tile::Wall => WALL_BG,
                Tile::Floor => FLOOR_BG,
            };
            console.set_cell(x, y, '\x20', Color::Default, bg);
        }
    }
}

pub fn render_player(console: &mut Console, game: &Game) {
    let (x, y) = game.player();
    console.print_char(x, y, '@', PLAYER_FG);
    console.show_cursor(x, y);
}
