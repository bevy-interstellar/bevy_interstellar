use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::AsBindGroup;

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Copy)]
#[uuid = "89156151-c061-4692-9b30-ca814ecf844a"]
pub struct SolarSystemMaterial {}

/// TODO: custom shaders
impl Material for SolarSystemMaterial {}
