use crate::console::{Color, Console};
use crate::logic::{Game, Glyph, Pos};

use super::theme::Theme;

pub fn render_map(console: &mut Console, theme: Theme, game: &Game) {
    for y in 0..game.level().height() {
        for x in 0..game.level().width() {
            let pos = Pos { x, y };
            let tile = game.level().get_tile(pos);
            if game.level().is_visible(pos) {
                console.set_cell(
                    x,
                    y,
                    theme.visible_ch(tile),
                    theme.visible_fg(tile),
                    theme.visible_bg(tile),
                );
            } else if game.level().is_explored(pos) {
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
}

pub fn render_entities(console: &mut Console, theme: Theme, game: &Game) {
    let visible_entities: Vec<_> = (game.world().positions())
        .filter(|&(_, &p)| game.level().is_visible(p))
        .map(|(e, &p)| (e, p, game.world().get_glyph(e)))
        .collect();
    for &(_, pos, glyph) in &visible_entities {
        console.print_char(pos.x, pos.y, theme.entity_ch(glyph), theme.entity_fg(glyph));
    }
    for &(_, pos, glyph) in &visible_entities {
        if glyph != Glyph::Corpse {
            console.print_char(pos.x, pos.y, theme.entity_ch(glyph), theme.entity_fg(glyph));
        }
    }
}

pub fn render_player(console: &mut Console, _theme: Theme, game: &Game) {
    let player = game.player();
    let pos = game.world().get_position(player);
    console.show_cursor(pos.x, pos.y);
}

pub fn render_hp(console: &mut Console, _theme: Theme, game: &Game) {
    let player = game.player();
    let stats = game.world().get_stats(player);
    let text = format!("HP: {}/{}", stats.hp(), stats.max_hp());
    console.print_str(1, 47, &text, Color::Default);
}
