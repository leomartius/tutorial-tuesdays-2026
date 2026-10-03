//! Entity definitions.

use super::Glyph;

pub enum ActorKind {
    Player,
    Orc,
    Troll,
}

pub struct ActorDef {
    pub name: &'static str,
    pub glyph: Glyph,
}

impl ActorKind {
    pub fn def(&self) -> ActorDef {
        match self {
            ActorKind::Player => ActorDef {
                name: "Player",
                glyph: Glyph::Player,
            },
            ActorKind::Orc => ActorDef {
                name: "Orc",
                glyph: Glyph::Orc,
            },
            ActorKind::Troll => ActorDef {
                name: "Troll",
                glyph: Glyph::Troll,
            },
        }
    }
}
