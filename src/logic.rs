mod actions;
mod definitions;
mod fov;
mod generate;
mod level;
mod world;

use actions::ActionIntent;
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

    pub fn player_command(&mut self, command: PlayerCommand) -> Result<(), ()> {
        let intent = command.into_intent(self.player);
        let plan = intent.validate(self).map_err(|_| ())?;
        plan.perform(self);
        let pov = self.world.get_position(self.player);
        self.level.update_vision(pov);
        for entity in self.world.entities() {
            if entity != self.player {
                let entity_name = self.world.get_name(entity);
                eprintln!("The {entity_name} wonders when it will get to take a real turn.");
            }
        }
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

pub enum PlayerCommand {
    Bump(i32, i32),
    Wait,
}

impl PlayerCommand {
    fn into_intent(self, actor: Entity) -> ActionIntent {
        use PlayerCommand::*;
        match self {
            Bump(dx, dy) => ActionIntent::Bump { actor, dx, dy },
            Wait => ActionIntent::Wait { actor },
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
    world.set_name(actor, def.name);
    actor
}
