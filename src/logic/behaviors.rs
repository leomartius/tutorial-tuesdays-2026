use super::Game;
use super::actions::ActionIntent;
use super::world::Entity;

pub enum BehaviorDef {
    Hostile,
}

impl BehaviorDef {
    pub fn instantiate(&self) -> Behavior {
        use BehaviorDef::*;
        match self {
            Hostile => Behavior::Hostile(HostileBehavior {}),
        }
    }
}

pub enum Behavior {
    Hostile(HostileBehavior),
}

impl Behavior {
    pub fn take_turn(&self, game: &Game, actor: Entity) -> ActionIntent {
        use Behavior::*;
        match self {
            Hostile(ai) => ai.take_turn(game, actor),
        }
    }
}

pub struct HostileBehavior {
    // TODO
}

impl HostileBehavior {
    fn take_turn(&self, game: &Game, actor: Entity) -> ActionIntent {
        let target = game.player();
        let actor_pos = game.world().get_position(actor);
        let target_pos = game.world().get_position(target);
        if game.level.is_adjacent_to(actor_pos, target_pos) {
            let (dx, dy) = actor_pos.delta(target_pos);
            return ActionIntent::Melee { actor, dx, dy };
        }
        if game.level().is_visible(actor_pos) {
            let dx = (target_pos.x - actor_pos.x).signum();
            let dy = (target_pos.y - actor_pos.y).signum();
            return ActionIntent::Move { actor, dx, dy };
        }
        ActionIntent::Wait { actor }
    }
}
