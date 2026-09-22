use super::{Game, Vec2, world::Entity};

pub enum Action {
    Move { dx: i32, dy: i32 },
}

impl Action {
    pub fn move_by(dx: i32, dy: i32) -> Self {
        Action::Move { dx, dy }
    }

    pub fn validate(&self, actor: Entity, game: &Game) -> Result<(), ()> {
        match self {
            Action::Move { dx, dy } => {
                let pos = game.world.get_position(actor) + Vec2 { x: *dx, y: *dy };
                if game.level.in_bounds(pos)
                    && game.level.is_walkable(pos)
                    && !game.world.is_occupied(pos)
                {
                    return Ok(());
                }
                Err(())
            }
        }
    }

    pub fn perform(&self, actor: Entity, game: &mut Game) {
        match self {
            Action::Move { dx, dy } => {
                let pos = game.world.get_position(actor) + Vec2 { x: *dx, y: *dy };
                game.world.set_position(actor, pos);
            }
        }
    }
}
