use core::fmt;

use bevy::ecs::prelude::*;
use bevy::log::*;
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// store the id for an game object, it's stable across the game session. this
/// is necessary because entity id provided by bevy is not consistent across
/// game session. internally it's implemented by uuid. the resource
/// `OIdTable` will store the translation between object id and entity id.
/// however, the updated may be delayed, so the order of system execution is
/// important if such translation is required.
///
/// S/L data
#[derive(Component, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Oid(Uuid);

impl fmt::Debug for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Default for Oid {
    fn default() -> Self {
        Self::nil()
    }
}

impl Oid {
    pub fn nil() -> Self {
        Self(Uuid::nil())
    }

    /// create a Oid based on randomness, backed by UUID V4
    pub fn v4() -> Self {
        let id = Self(Uuid::new_v4());
        info!("generate v4 id {:?}", id);
        id
    }

    /// create a Oid based on hash value of the data, backed by UUID V5
    pub fn v5(data: &[u8]) -> Self {
        let id = Self(Uuid::new_v5(&Uuid::NAMESPACE_OID, data));
        info!("generate v5 id {:?}", id);
        id
    }
}

/// a resource to keep track of the mapping between `StableId` and `Entity`.
///
/// CON data
#[derive(Resource, Debug, Clone)]
pub struct OidTable {
    data: FxHashMap<Oid, Entity>,
    rebuild_counter: usize,
}

impl Default for OidTable {
    fn default() -> Self {
        Self::new()
    }
}

impl OidTable {
    /// create a empty table
    pub fn new() -> Self {
        OidTable {
            data: Default::default(),
            rebuild_counter: 0,
        }
    }

    /// get the `Entity` corresponding to `OId`
    pub fn query(&self, id: &Oid) -> Option<Entity> {
        self.data.get(id).cloned()
    }

    /// insert a `OId`, `Entity` pair to the table
    pub fn insert(&mut self, id: Oid, entity: Entity) {
        info!("insert {:?} - {:?} into id table.", id, entity);
        self.data.insert(id, entity);
    }

    /// record the number of should removed entry. since it's ok to ignore this
    /// request until there is memory issue, we ignore it until the counter grow
    /// too large.
    pub fn remove_entry(&mut self, i: usize) {
        self.rebuild_counter += i;
    }

    /// test if we should rebuild the table.
    pub fn should_rebuild(&self) -> bool {
        return self.rebuild_counter > self.data.len();
    }

    /// rebuild the table
    pub fn rebuild(&mut self, iter: impl Iterator<Item = (Entity, Oid)>) {
        self.data.clear();
        self.rebuild_counter = 0;

        for (eid, oid) in iter {
            self.insert(oid, eid);
        }
    }
}

/// a system for keeping track of `OIdTable`
///
/// schedule requirement:
/// - must run after any modification for `OId`
/// - must run before `oid_table_rebuild_system`
/// - must run before any conversion from `OId` to `Entity`
pub fn oid_table_update_system(
    changed: Query<(Entity, &Oid), Changed<Oid>>,
    removed: RemovedComponents<Oid>,
    mut table: ResMut<OidTable>,
) {
    for id in changed.iter() {
        table.as_mut().insert(*id.1, id.0)
    }

    table.as_mut().remove_entry(removed.iter().len());
}

pub fn oid_table_rebuild_system(all: Query<(Entity, &Oid)>, mut table: ResMut<OidTable>) {
    if table.as_ref().should_rebuild() {
        table.as_mut().rebuild(all.iter().map(|e| (e.0, *e.1)));
    }
}
