mod actions;
mod behaviors;
mod definitions;
mod fov;
mod generate;
mod level;
mod world;

use actions::{ActionIntent, ActionPlan};
use behaviors::Behavior;
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
        self.end_turn();
        Ok(())
    }

    fn end_turn(&mut self) {
        let pov = self.world.get_position(self.player);
        self.level.update_vision(pov);
        self.handle_enemy_turns();
    }

    fn handle_enemy_turns(&mut self) {
        let actors: Vec<_> = self.world.behaviors().map(|(e, _)| e).collect();
        for actor in actors {
            let intent = Behavior::take_turn(self, actor);
            let plan = intent.validate(self).unwrap_or(ActionPlan::Wait { actor });
            plan.perform(self);
        }
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

    pub fn delta(self, other: Self) -> (i32, i32) {
        (other.x - self.x, other.y - self.y)
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
    if let Some(behavior_def) = def.behavior {
        world.set_behavior(actor, behavior_def.instantiate());
    }
    actor
}
