use crate::console::{Console, Event};
use crate::logic::Game;

use super::input::*;
use super::render::*;
use super::{Scene, Transition, theme::Theme};

pub struct GameScene;

impl Scene for GameScene {
    fn render(&self, game: &Game, console: &mut Console, theme: Theme) {
        render_map(console, theme, game);
        render_entities(console, theme, game);
        render_player(console, theme, game);
        render_hp(console, theme, game);
    }

    fn handle_event(&mut self, game: &mut Game, event: Event) -> Transition {
        match map_play_command(event) {
            Some(Command::PlayerAction(action)) => match game.submit_player_action(action) {
                Ok(_) => {
                    if game.game_over() {
                        Transition::Push(Box::new(GameOver))
                    } else {
                        Transition::Okay
                    }
                }
                Err(_) => Transition::Beep,
            },
            None => Transition::Beep,
        }
    }
}

pub struct GameOver;

impl Scene for GameOver {
    fn render(&self, _game: &Game, _console: &mut Console, _theme: Theme) {}

    fn handle_event(&mut self, _game: &mut Game, _event: Event) -> Transition {
        Transition::Beep
    }
}
