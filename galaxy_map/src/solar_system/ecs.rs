use crate::solar_system::id::SolarSystemIdGenerator;
use crate::solar_system::render::SolarSystemMaterial;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use utils::{id::ObjectIdGenerator, prelude::*};

#[derive(Bundle, Clone)]
pub struct SolarSystemBundle {
    /// id
    pub id: ObjectId,

    /// position
    pub transform: Transform,
    pub global_transform: GlobalTransform,

    /// rendering related
    pub mesh: Handle<Mesh>,
    pub material: Handle<SolarSystemMaterial>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub layer: RenderLayers,
}

static ID: SolarSystemIdGenerator = SolarSystemIdGenerator::new();

impl Default for SolarSystemBundle {
    fn default() -> Self {
        Self {
            id: ID.spawn(),
            transform: Default::default(),
            global_transform: Default::default(),
            mesh: Default::default(),
            material: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            layer: RenderLayers::layer(0),
        }
    }
}
