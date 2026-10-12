use super::{Game, Pos, combat, world::Entity};

pub enum ActionKind {
    Bump(i32, i32),
    Melee(i32, i32),
    Move(i32, i32),
    Wait,
}

impl ActionKind {
    pub fn into_intent(self, actor: Entity) -> ActionIntent {
        ActionIntent { actor, kind: self }
    }
}

pub struct ActionIntent {
    actor: Entity,
    kind: ActionKind,
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
        let actor = self.actor;
        use ActionKind::*;
        match self.kind {
            Bump(dx, dy) => validate_bump(game, actor, dx, dy),
            Melee(dx, dy) => validate_melee(game, actor, dx, dy),
            Move(dx, dy) => validate_move(game, actor, dx, dy),
            Wait => validate_wait(game, actor),
        }
    }
}

fn validate_bump(game: &Game, actor: Entity, dx: i32, dy: i32) -> Result<ActionPlan, ActionError> {
    let dest = game.world.get_position(actor).offset(dx, dy);
    if game.world.is_blocked(dest) {
        validate_melee(game, actor, dx, dy)
    } else {
        validate_move(game, actor, dx, dy)
    }
}

fn validate_melee(game: &Game, actor: Entity, dx: i32, dy: i32) -> Result<ActionPlan, ActionError> {
    let actor_pos = game.world.get_position(actor);
    let dest = actor_pos.offset(dx, dy);
    if !game.level.in_bounds(dest) {
        return Err(ActionError::DestinationOutOfBounds);
    }
    if !game.level.is_adjacent_to(actor_pos, dest) {
        return Err(ActionError::DestinationNotAdjacent);
    }
    match game.world.blocker_at(dest) {
        Some(target) => Ok(ActionPlan::Melee { actor, target }),
        None => Err(ActionError::NoTarget),
    }
}

fn validate_move(game: &Game, actor: Entity, dx: i32, dy: i32) -> Result<ActionPlan, ActionError> {
    let actor_pos = game.world.get_position(actor);
    let dest = actor_pos.offset(dx, dy);
    if !game.level.in_bounds(dest) {
        return Err(ActionError::DestinationOutOfBounds);
    }
    if !game.level.is_adjacent_to(actor_pos, dest) {
        return Err(ActionError::DestinationNotAdjacent);
    }
    if !game.level.is_walkable(dest) || game.world.is_blocked(dest) {
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
    combat::melee_attack(game, actor, target);
}

fn perform_move(game: &mut Game, actor: Entity, dest: Pos) {
    game.world.set_position(actor, dest);
}

fn perform_wait(_game: &mut Game, _actor: Entity) {}
