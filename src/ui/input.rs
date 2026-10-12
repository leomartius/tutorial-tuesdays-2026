use crate::console::{Event, Key};
use crate::logic::ActionKind;

pub enum Command {
    PlayerAction(ActionKind),
}

pub fn map_play_command(event: Event) -> Option<Command> {
    let command = match event {
        Event::KeyChar('y') => Command::PlayerAction(ActionKind::Bump(-1, -1)),
        Event::KeyChar('k') => Command::PlayerAction(ActionKind::Bump(0, -1)),
        Event::KeyChar('u') => Command::PlayerAction(ActionKind::Bump(1, -1)),
        Event::KeyChar('h') => Command::PlayerAction(ActionKind::Bump(-1, 0)),
        Event::KeyChar('l') => Command::PlayerAction(ActionKind::Bump(1, 0)),
        Event::KeyChar('b') => Command::PlayerAction(ActionKind::Bump(-1, 1)),
        Event::KeyChar('j') => Command::PlayerAction(ActionKind::Bump(0, 1)),
        Event::KeyChar('n') => Command::PlayerAction(ActionKind::Bump(1, 1)),
        Event::KeySpecial(Key::Home) => Command::PlayerAction(ActionKind::Bump(-1, -1)),
        Event::KeySpecial(Key::Up) => Command::PlayerAction(ActionKind::Bump(0, -1)),
        Event::KeySpecial(Key::PgUp) => Command::PlayerAction(ActionKind::Bump(1, -1)),
        Event::KeySpecial(Key::Left) => Command::PlayerAction(ActionKind::Bump(-1, 0)),
        Event::KeySpecial(Key::Right) => Command::PlayerAction(ActionKind::Bump(1, 0)),
        Event::KeySpecial(Key::End) => Command::PlayerAction(ActionKind::Bump(-1, 1)),
        Event::KeySpecial(Key::Down) => Command::PlayerAction(ActionKind::Bump(0, 1)),
        Event::KeySpecial(Key::PgDn) => Command::PlayerAction(ActionKind::Bump(1, 1)),
        Event::KeyChar('.') => Command::PlayerAction(ActionKind::Wait),
        _ => return None,
    };
    Some(command)
}
