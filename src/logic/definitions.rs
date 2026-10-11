//! Entity definitions.

use super::Glyph;
use super::behaviors::BehaviorDef;
use super::combat::StatsDef;

pub enum ActorKind {
    Player,
    Orc,
    Troll,
}

pub struct ActorDef {
    pub name: &'static str,
    pub glyph: Glyph,
    pub behavior: Option<BehaviorDef>,
    pub stats: Option<StatsDef>,
}

impl ActorKind {
    pub fn def(&self) -> ActorDef {
        match self {
            ActorKind::Player => ActorDef {
                name: "Player",
                glyph: Glyph::Player,
                behavior: None,
                stats: Some(StatsDef {
                    max_hp: 30,
                    defense: 2,
                    power: 5,
                }),
            },
            ActorKind::Orc => ActorDef {
                name: "Orc",
                glyph: Glyph::Orc,
                behavior: Some(BehaviorDef::Hostile),
                stats: Some(StatsDef {
                    max_hp: 10,
                    defense: 0,
                    power: 3,
                }),
            },
            ActorKind::Troll => ActorDef {
                name: "Troll",
                glyph: Glyph::Troll,
                behavior: Some(BehaviorDef::Hostile),
                stats: Some(StatsDef {
                    max_hp: 16,
                    defense: 1,
                    power: 4,
                }),
            },
        }
    }
}
