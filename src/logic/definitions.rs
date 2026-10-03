//! Entity definitions.

use super::Glyph;
use super::behaviors::BehaviorDef;

pub enum ActorKind {
    Player,
    Orc,
    Troll,
}

pub struct ActorDef {
    pub name: &'static str,
    pub glyph: Glyph,
    pub behavior: Option<BehaviorDef>,
}

impl ActorKind {
    pub fn def(&self) -> ActorDef {
        match self {
            ActorKind::Player => ActorDef {
                name: "Player",
                glyph: Glyph::Player,
                behavior: None,
            },
            ActorKind::Orc => ActorDef {
                name: "Orc",
                glyph: Glyph::Orc,
                behavior: Some(BehaviorDef::Hostile),
            },
            ActorKind::Troll => ActorDef {
                name: "Troll",
                glyph: Glyph::Troll,
                behavior: Some(BehaviorDef::Hostile),
            },
        }
    }
}
