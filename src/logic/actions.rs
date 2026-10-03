use super::{Game, Pos, world::Entity};

pub enum ActionIntent {
    Bump { actor: Entity, dx: i32, dy: i32 },
    Melee { actor: Entity, dx: i32, dy: i32 },
    Move { actor: Entity, dx: i32, dy: i32 },
    Wait { actor: Entity },
}

pub enum ActionPlan {
    Melee { actor: Entity, target: Entity },
    Move { actor: Entity, dest: Pos },
    Wait { actor: Entity },
}

pub enum ActionError {
    DestinationBlocked,
    DestinationNotAdjacent,
    DestinationOutOfBounds,
    NoTarget,
}

impl ActionIntent {
    pub fn validate(self, game: &Game) -> Result<ActionPlan, ActionError> {
        use ActionIntent::*;
        match self {
            Bump { actor, dx, dy } => validate_bump(game, actor, dx, dy),
            Melee { actor, dx, dy } => validate_melee(game, actor, dx, dy),
            Move { actor, dx, dy } => validate_move(game, actor, dx, dy),
            Wait { actor } => validate_wait(game, actor),
        }
    }
}

fn validate_bump(game: &Game, actor: Entity, dx: i32, dy: i32) -> Result<ActionPlan, ActionError> {
    let dest = game.world.get_position(actor).offset(dx, dy);
    match game.world.entity_at(dest) {
        Some(_) => validate_melee(game, actor, dx, dy),
        None => validate_move(game, actor, dx, dy),
    }
}

fn validate_melee(game: &Game, actor: Entity, dx: i32, dy: i32) -> Result<ActionPlan, ActionError> {
    let dest = game.world.get_position(actor).offset(dx, dy);
    match game.world.entity_at(dest) {
        Some(target) => Ok(ActionPlan::Melee { actor, target }),
        None => Err(ActionError::NoTarget),
    }
}

fn validate_move(game: &Game, actor: Entity, dx: i32, dy: i32) -> Result<ActionPlan, ActionError> {
    let actor_pos = game.world.get_position(actor);
    let dest = actor_pos.offset(dx, dy);
    if !game.level.is_adjacent_to(actor_pos, dest) {
        return Err(ActionError::DestinationNotAdjacent);
    }
    if !game.level.in_bounds(dest) {
        return Err(ActionError::DestinationOutOfBounds);
    }
    if !game.level.is_walkable(dest) || game.world.is_occupied(dest) {
        return Err(ActionError::DestinationBlocked);
    }
    Ok(ActionPlan::Move { actor, dest })
}

fn validate_wait(_game: &Game, actor: Entity) -> Result<ActionPlan, ActionError> {
    Ok(ActionPlan::Wait { actor })
}

impl ActionPlan {
    pub fn perform(self, game: &mut Game) {
        use ActionPlan::*;
        match self {
            Melee { actor, target } => perform_melee(game, actor, target),
            Move { actor, dest } => perform_move(game, actor, dest),
            Wait { actor } => perform_wait(game, actor),
        }
    }
}

fn perform_melee(game: &mut Game, actor: Entity, target: Entity) {
    let actor_name = game.world.get_name(actor);
    let target_name = game.world.get_name(target);
    eprintln!("The {actor_name} attacks the {target_name}!");
}

fn perform_move(game: &mut Game, actor: Entity, dest: Pos) {
    game.world.set_position(actor, dest);
}

fn perform_wait(_game: &mut Game, _actor: Entity) {}
