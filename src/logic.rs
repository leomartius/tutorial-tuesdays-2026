mod level;
mod world;

use std::ops::Add;

use level::Level;
use world::{Entity, World};

pub struct Game {
    level: Level,
    world: World,
    player: Entity,
}

impl Game {
    pub fn new() -> Self {
        let mut level = Level::new(80, 45);
        for x in 30..33 {
            level.set_tile(Vec2 { x, y: 22 }, Tile::Wall);
        }
        let mut world = World::new();
        let player = world.spawn();
        world.set_glyph(player, Glyph::Player);
        world.set_position(player, Vec2 { x: 40, y: 25 });
        let npc = world.spawn();
        world.set_glyph(npc, Glyph::Npc);
        world.set_position(npc, Vec2 { x: 35, y: 25 });
        Self {
            level,
            world,
            player,
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) -> Result<(), ()> {
        let pos = self.world.get_position(self.player) + Vec2 { x: dx, y: dy };
        if self.level.in_bounds(pos) && self.level.is_walkable(pos) {
            self.world.set_position(self.player, pos);
            return Ok(());
        }
        Err(())
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
