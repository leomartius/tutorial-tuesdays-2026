use super::{Game, Glyph, world::Entity};

pub struct StatsDef {
    pub max_hp: i32,
    pub defense: i32,
    pub power: i32,
}

impl StatsDef {
    pub fn instantiate(self) -> Stats {
        Stats {
            hp: self.max_hp,
            max_hp: self.max_hp,
            defense: self.defense,
            power: self.power,
        }
    }
}

pub struct Stats {
    hp: i32,
    max_hp: i32,
    defense: i32,
    power: i32,
}

impl Stats {
    pub fn hp(&self) -> i32 {
        self.hp
    }

    pub fn max_hp(&self) -> i32 {
        self.max_hp
    }

    pub fn dead(&self) -> bool {
        self.hp == 0
    }

    fn harm(&mut self, damage: i32) {
        debug_assert!(damage >= 0);
        self.hp = (self.hp - damage).max(0);
    }
}

pub fn melee_attack(game: &mut Game, attacker: Entity, defender: Entity) {
    let atk_stats = game.world.get_stats(attacker);
    let atk_power = atk_stats.power;
    let def_stats = game.world.get_stats_mut(defender);
    let damage = (atk_power - def_stats.defense).max(0);
    def_stats.harm(damage);

    let atk_name = game.world.get_name(attacker);
    let def_name = game.world.get_name(defender);
    if damage > 0 {
        eprintln!("{atk_name} attacks {def_name} for {damage} hit points.");
    } else {
        eprintln!("{atk_name} attacks {def_name} but does no damage.");
    }

    let def_stats = game.world.get_stats_mut(defender);
    if def_stats.dead() {
        let def_name = game.world.get_name(defender);
        eprintln!("{def_name} is dead!");
        kill(game, defender);
    }
}

fn kill(game: &mut Game, actor: Entity) {
    game.world.remove_behavior(actor);
    game.world.remove_blocker(actor);
    game.world.set_glyph(actor, Glyph::Corpse);
}
