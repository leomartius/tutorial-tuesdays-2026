//! Entity definitions.

use super::Glyph;

pub enum ActorKind {
    Player,
    Orc,
    Troll,
}

pub struct ActorDef {
    pub glyph: Glyph,
}

impl ActorKind {
    pub fn def(&self) -> ActorDef {
        match self {
            ActorKind::Player => ActorDef {
                glyph: Glyph::Player,
            },
            ActorKind::Orc => ActorDef {
                glyph: Glyph::Orc,
            },
            ActorKind::Troll => ActorDef {
                glyph: Glyph::Troll,
            },
        }
    }
}
