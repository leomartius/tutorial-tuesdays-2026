mod action;
mod definitions;
mod fov;
mod generate;
mod level;
mod world;

use std::ops::Add;

pub use action::Action;
use definitions::ActorKind;
use level::Level;
use world::{Entity, World};

pub struct Game {
    level: Level,
    world: World,
    player: Entity,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();
        let player = spawn_actor(&mut world, ActorKind::Player, None);
        let mut level = generate::generate_level(&mut world, player);
        level.update_vision(world.get_position(player));
        Self {
            level,
            world,
            player,
        }
    }

    pub fn player_action(&mut self, action: Action) -> Result<(), ()> {
        action.validate(self.player, self)?;
        action.perform(self.player, self);
        let pov = self.world.get_position(self.player);
        self.level.update_vision(pov);
        Ok(())
    }

    pub fn level(&self) -> &Level {
        &self.level
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn player(&self) -> Entity {
        self.player
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
}

#[derive(Clone, Copy)]
pub enum Glyph {
    Player,
    Orc,
    Troll,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Vec2 {
    fn from(tuple: (i32, i32)) -> Self {
        Vec2 {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn spawn_actor(world: &mut World, kind: ActorKind, pos: Option<Vec2>) -> Entity {
    let actor = world.spawn();
    if let Some(pos) = pos {
        world.set_position(actor, pos);
    }
    let def = kind.def();
    world.set_glyph(actor, def.glyph);
    actor
}
