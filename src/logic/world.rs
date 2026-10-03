use std::collections::HashMap;

use slotmap::{SecondaryMap, SlotMap, new_key_type};

use super::behaviors::Behavior;
use super::{Glyph, Pos};

new_key_type! {
    pub struct Entity;
}

#[derive(Default)]
pub struct World {
    // entity ID
    entities: SlotMap<Entity, ()>,
    // entity position + reverse spatial index
    positions: SecondaryMap<Entity, Pos>,
    occupancy: HashMap<Pos, Entity>,
    // other components
    glyphs: SecondaryMap<Entity, Glyph>,
    names: SecondaryMap<Entity, &'static str>,
    behaviors: SecondaryMap<Entity, Behavior>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entities(&self) -> impl Iterator<Item = Entity> {
        self.entities.keys()
    }

    pub fn spawn(&mut self) -> Entity {
        self.entities.insert(())
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.entities.remove(entity);
        let old_pos = self.positions.remove(entity);
        if let Some(old_pos) = old_pos {
            self.occupancy.remove(&old_pos);
        }
        self.glyphs.remove(entity);
        self.names.remove(entity);
        self.behaviors.remove(entity);
    }

    pub fn get_position(&self, entity: Entity) -> Pos {
        debug_assert!(self.positions.contains_key(entity));
        self.positions[entity]
    }

    pub fn set_position(&mut self, entity: Entity, pos: Pos) {
        let old_pos = self.positions.insert(entity, pos);
        if let Some(old_pos) = old_pos {
            self.occupancy.remove(&old_pos);
        }
        debug_assert!(!self.occupancy.contains_key(&pos));
        self.occupancy.insert(pos, entity);
    }

    pub fn entity_at(&self, pos: Pos) -> Option<Entity> {
        self.occupancy.get(&pos).copied()
    }

    pub fn is_occupied(&self, pos: Pos) -> bool {
        self.occupancy.contains_key(&pos)
    }

    pub fn get_glyph(&self, entity: Entity) -> Glyph {
        debug_assert!(self.glyphs.contains_key(entity));
        self.glyphs[entity]
    }

    pub fn set_glyph(&mut self, entity: Entity, glyph: Glyph) {
        self.glyphs.insert(entity, glyph);
    }

    pub fn get_name(&self, entity: Entity) -> &str {
        debug_assert!(self.glyphs.contains_key(entity));
        self.names[entity]
    }

    pub fn set_name(&mut self, entity: Entity, name: &'static str) {
        self.names.insert(entity, name);
    }

    pub fn behaviors(&self) -> impl Iterator<Item = (Entity, &Behavior)> {
        self.behaviors.iter()
    }

    pub fn get_behavior(&self, entity: Entity) -> &Behavior {
        debug_assert!(self.behaviors.contains_key(entity));
        &self.behaviors[entity]
    }

    pub fn set_behavior(&mut self, entity: Entity, behavior: Behavior) {
        self.behaviors.insert(entity, behavior);
    }
}
