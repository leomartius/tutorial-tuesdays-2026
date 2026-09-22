mod action;
mod generate;
mod level;
mod world;

use std::ops::Add;

pub use action::Action;
use level::Level;
use world::{Entity, World};

pub struct Game {
    level: Level,
    world: World,
    player: Entity,
}

impl Game {
    pub fn new() -> Self {
        let level = generate::generate_level();
        let mut world = World::new();
        let player = world.spawn();
        world.set_glyph(player, Glyph::Player);
        world.set_position(player, level.entry());
        let npc = world.spawn();
        world.set_glyph(npc, Glyph::Npc);
        world.set_position(npc, Vec2 { x: 35, y: 25 });
        Self {
            level,
            world,
            player,
        }
    }

    pub fn player_action(&mut self, action: Action) -> Result<(), ()> {
        action.validate(self.player, self)?;
        action.perform(self.player, self);
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Glyph {
    Player,
    Npc,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
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
