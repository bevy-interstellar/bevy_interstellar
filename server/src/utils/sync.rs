use bevy::prelude::{Component, Entity};

#[derive(Component, Debug, Clone, Copy)]
pub struct SyncTo(Entity);

impl SyncTo {
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }
}

impl From<SyncTo> for Entity {
    fn from(value: SyncTo) -> Self {
        value.0
    }
}
