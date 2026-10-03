mod action;
mod definitions;
mod fov;
mod generate;
mod level;
mod world;

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
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn offset(self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl From<(i32, i32)> for Pos {
    fn from(tuple: (i32, i32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

fn spawn_actor(world: &mut World, kind: ActorKind, pos: Option<Pos>) -> Entity {
    let actor = world.spawn();
    if let Some(pos) = pos {
        world.set_position(actor, pos);
    }
    let def = kind.def();
    world.set_glyph(actor, def.glyph);
    actor
}
