use std::collections::HashMap;

use slotmap::{SecondaryMap, SlotMap, new_key_type};

use super::{Glyph, Pos};

new_key_type! {
    pub struct Entity;
}

pub struct World {
    entities: SlotMap<Entity, ()>,
    positions: SecondaryMap<Entity, Pos>,
    occupancy: HashMap<Pos, Entity>,
    glyphs: SecondaryMap<Entity, Glyph>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: SlotMap::with_key(),
            positions: SecondaryMap::new(),
            occupancy: HashMap::new(),
            glyphs: SecondaryMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.positions.clear();
        self.occupancy.clear();
        self.glyphs.clear();
    }

    pub fn entities(&self) -> slotmap::basic::Keys<'_, Entity, ()> {
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
}
