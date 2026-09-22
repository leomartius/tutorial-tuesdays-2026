use crate::console::Color;
use crate::logic::{Glyph, Tile};

#[derive(Clone, Copy)]
pub struct Theme;

impl Theme {
    pub fn explored_ch(&self, _tile: Tile) -> char {
        '\x20' // ASCII space
    }
    pub fn explored_fg(&self, _tile: Tile) -> Color {
        Color::Default
    }
    pub fn explored_bg(&self, tile: Tile) -> Color {
        match tile {
            Tile::Wall => Color::rgb(0, 0, 100),
            Tile::Floor => Color::rgb(50, 50, 150),
        }
    }

    pub fn entity_ch(&self, glyph: Glyph) -> char {
        match glyph {
            Glyph::Player => '@',
            Glyph::Npc => '@',
        }
    }
    pub fn entity_fg(&self, glyph: Glyph) -> Color {
        match glyph {
            Glyph::Player => Color::rgb(255, 255, 255),
            Glyph::Npc => Color::rgb(255, 255, 0),
        }
    }
}
