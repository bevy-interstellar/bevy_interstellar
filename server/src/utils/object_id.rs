use bevy::prelude::*;
use nohash_hasher;
use serde::{Deserialize, Serialize};

//////////////////////// Component ////////////////////////

/// a component to store a game object id that is stable across the game
/// session. note that the entity id is not stable across game session. A
/// `ObjectId` internal store u32, where the first 8 bits to indicate object
/// type and other metadata, while the last 24 bits to store the actual id.
///
/// this component should be serialized by translate from `Entity`.
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectId(u32);

impl std::hash::Hash for ObjectId {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        hasher.write_u32(self.0)
    }
}

impl nohash_hasher::IsEnabled for ObjectId {}

impl ObjectId {
    /// create a new `ObjectId` object from prefix & suffix
    pub fn new(r#type: u32, sub_id: u32) -> Self {
        info!("[utils] new object id {:#04x}-{:#08x}", r#type, sub_id);
        Self(r#type << 24 | sub_id)
    }

    /// create a new `ObjectId` object from u64.
    pub fn from_raw(bits: u32) -> Self {
        Self(bits)
    }

    /// a always invalid id.
    pub const fn invalid() -> Self {
        Self(u32::MAX)
    }

    /// the 8 bit suffix of the id, useful for object that is no more than
    /// 256 ids.
    pub fn sub_id_u8(&self) -> u8 {
        (self.0 & 0xff) as u8
    }

    /// the 16 bit suffix of the id, useful for object that is no more than
    /// 65536 ids.
    pub fn sub_id_u16(&self) -> u16 {
        (self.0 & 0xffff) as u16
    }

    /// the 24 bit suffix of the id, the actual id part.
    pub fn sub_id(&self) -> u32 {
        self.0 & 0xffffff
    }

    pub fn r#type(&self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub fn id(&self) -> u32 {
        self.0
    }
}

//////////////////////// Resource ////////////////////////

/// a resource to keep track of the mapping between `ObjectId` and `Entity`.
///
/// this item should be reconstructed on reload.
#[derive(Resource, Debug, Clone, Default)]
pub struct ObjectIdTable {
    data: nohash_hasher::IntMap<ObjectId, Entity>,
}

impl ObjectIdTable {
    /// create a empty table
    pub fn new() -> Self {
        Self::default()
    }

    /// get the `Entity` corresponding to `id`
    pub fn query(&self, id: &ObjectId) -> Option<Entity> {
        self.data.get(id).cloned()
    }

    /// insert a `ObjectId`, `Entity` pair to the table
    pub fn insert(&mut self, id: ObjectId, entity: Entity) {
        info!("[utils] insert {:?} - {:?} into id table.", id, entity);
        self.data.insert(id, entity);
    }

    /// remove a `ObjectId` entry, useful when a game object disappear
    pub fn remove(&mut self, id: ObjectId) {
        info!("[utils] remove {:?} from id table.", id);
        self.data.remove(&id);
    }

    /// clear all data in the table, useful when reload the game
    pub fn reset(&mut self) {
        info!("[utils] reset id table.");
        self.data.clear();
    }
}

//////////////////////// System ////////////////////////

/// a system for keeping track of `ObjectIdTable`
///
/// schedule requirement:
/// - must run before any conversion from `ObjectId` to `Entity`
/// - must run after any modification for `ObjectId`
pub fn object_id_table_update_system(
    added: Query<(Entity, &ObjectId), Added<ObjectId>>,
    changed: Query<(Entity, &ObjectId), Changed<ObjectId>>,
    mut table: ResMut<ObjectIdTable>,
) {
    for id in added.iter() {
        table.as_mut().insert(*id.1, id.0)
    }

    for id in changed.iter() {
        if *id.1 == ObjectId::invalid() {
            table.as_mut().remove(*id.1);
        } else {
            table.as_mut().insert(*id.1, id.0)
        }
    }
}

//////////////////////// Test ////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_id() {
        let id = ObjectId::from_raw(0x5678abcd);
        let id2 = ObjectId::new(0x56, 0x78abcd);

        assert_eq!(id, id2);

        assert_eq!(0x5678abcd, id.id());
        assert_eq!(0x00000056, id.r#type());
        assert_eq!(0x0078abcd, id.sub_id());
        assert_eq!(0x0000abcd, id.sub_id_u16());
        assert_eq!(0x000000cd, id.sub_id_u8());
    }
}
