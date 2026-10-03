use crate::console::{Event, Key};

use super::Command;

pub fn map_play_command(event: Event) -> Option<Command> {
    let command = match event {
        Event::KeyChar('y') => Command::Bump(-1, -1),
        Event::KeyChar('k') => Command::Bump(0, -1),
        Event::KeyChar('u') => Command::Bump(1, -1),
        Event::KeyChar('h') => Command::Bump(-1, 0),
        Event::KeyChar('l') => Command::Bump(1, 0),
        Event::KeyChar('b') => Command::Bump(-1, 1),
        Event::KeyChar('j') => Command::Bump(0, 1),
        Event::KeyChar('n') => Command::Bump(1, 1),
        Event::KeySpecial(Key::Home) => Command::Bump(-1, -1),
        Event::KeySpecial(Key::Up) => Command::Bump(0, -1),
        Event::KeySpecial(Key::PgUp) => Command::Bump(1, -1),
        Event::KeySpecial(Key::Left) => Command::Bump(-1, 0),
        Event::KeySpecial(Key::Right) => Command::Bump(1, 0),
        Event::KeySpecial(Key::End) => Command::Bump(-1, 1),
        Event::KeySpecial(Key::Down) => Command::Bump(0, 1),
        Event::KeySpecial(Key::PgDn) => Command::Bump(1, 1),
        Event::KeyChar('.') => Command::Wait,
        _ => return None,
    };
    Some(command)
}
