use crate::console::Console;
use crate::logic::{Game, Vec2};

use super::theme::Theme;

pub fn render_map(console: &mut Console, theme: Theme, game: &Game) {
    for y in 0..game.level().height() {
        for x in 0..game.level().width() {
            let tile = game.level().get_tile(Vec2 { x, y });
            console.set_cell(
                x,
                y,
                theme.explored_ch(tile),
                theme.explored_fg(tile),
                theme.explored_bg(tile),
            );
        }
    }
}

pub fn render_entities(console: &mut Console, theme: Theme, game: &Game) {
    for e in game.world().entities() {
        let pos = game.world().get_position(e);
        let glyph = game.world().get_glyph(e);
        console.print_char(pos.x, pos.y, theme.entity_ch(glyph), theme.entity_fg(glyph));
    }
}

pub fn render_player(console: &mut Console, _theme: Theme, game: &Game) {
    let player = game.player();
    let pos = game.world().get_position(player);
    console.show_cursor(pos.x, pos.y);
}
