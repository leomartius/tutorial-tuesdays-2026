use super::actions::ActionIntent;
use super::world::Entity;
use super::{Game, Pos};

pub enum BehaviorDef {
    Hostile,
}

impl BehaviorDef {
    pub fn instantiate(&self) -> Behavior {
        use BehaviorDef::*;
        match self {
            Hostile => Behavior::Hostile(HostileBehavior::default()),
        }
    }
}

pub enum Behavior {
    Hostile(HostileBehavior),
}

impl Behavior {
    pub fn take_turn(game: &mut Game, actor: Entity) -> ActionIntent {
        use Behavior::*;
        match game.world.get_behavior(actor) {
            Hostile(_) => HostileBehavior::take_turn(game, actor),
        }
    }
}

#[derive(Default)]
pub struct HostileBehavior {
    path: Vec<Pos>,
    index: usize,
}

impl HostileBehavior {
    fn take_turn(game: &mut Game, actor: Entity) -> ActionIntent {
        let target = game.player();
        let actor_pos = game.world().get_position(actor);
        let target_pos = game.world().get_position(target);
        if game.level.is_adjacent_to(actor_pos, target_pos) {
            let (dx, dy) = actor_pos.delta(target_pos);
            return ActionIntent::Melee { actor, dx, dy };
        }
        if game.level.is_visible(actor_pos) {
            let path = pathfinding::find_path(actor_pos, target_pos, &game.level, &game.world);
            if let Some(path) = path {
                debug_assert!(path.len() > 2);
                let Behavior::Hostile(state) = game.world.get_behavior_mut(actor);
                state.path = path;
                state.index = 0;
            }
        }
        let Behavior::Hostile(state) = game.world.get_behavior_mut(actor);
        if let Some(&prev) = state.path.get(state.index)
            && prev == actor_pos
        {
            state.index += 1;
        }
        if let Some(&next) = state.path.get(state.index) {
            let (dx, dy) = actor_pos.delta(next);
            return ActionIntent::Move { actor, dx, dy };
        }
        ActionIntent::Wait { actor }
    }
}

mod pathfinding {
    use std::cmp::{max, min};

    use pathfinding::prelude::astar;

    use crate::logic::{Pos, level::Level, world::World};

    fn walkable_neighbors(pos: &Pos, level: &Level, world: &World) -> Vec<(Pos, i32)> {
        vec![
            (pos.offset(1, -1), 3),
            (pos.offset(1, 1), 3),
            (pos.offset(-1, 1), 3),
            (pos.offset(-1, -1), 3),
            (pos.offset(0, -1), 2),
            (pos.offset(1, 0), 2),
            (pos.offset(0, 1), 2),
            (pos.offset(-1, 0), 2),
        ]
        .into_iter()
        .filter(|&(p, _)| level.is_walkable(p))
        .map(|(p, c)| (p, c * if world.is_occupied(p) { 11 } else { 1 }))
        .collect()
    }

    fn heuristic(pos: &Pos, goal: &Pos) -> i32 {
        let dx = (goal.x - pos.x).abs();
        let dy = (goal.y - pos.y).abs();
        max(dx, dy) * 2 + min(dx, dy)
    }

    pub fn find_path(start: Pos, goal: Pos, level: &Level, world: &World) -> Option<Vec<Pos>> {
        let result = astar(
            &start,
            |p| walkable_neighbors(p, level, world),
            |p| heuristic(p, &goal),
            |p| *p == goal,
        );
        match result {
            Some((path, ..)) => {
                debug_assert!(path[0] == start);
                debug_assert!(path[path.len() - 1] == goal);
                Some(path)
            }
            None => None,
        }
    }
}
