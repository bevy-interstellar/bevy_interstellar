use bevy::prelude::Material;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::AsBindGroup;

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Copy, Default)]
#[uuid = "4291d443-7434-4be9-915a-e07880b657b9"]
pub struct HiResSunMaterial {
    #[uniform(0)]
    radius: f32,
    #[uniform(0)]
    luminosity: f32,
    #[uniform(0)]
    temperature: f32,
}

impl HiResSunMaterial {
    pub fn new(radius: f32, luminosity: f32, temperature: f32) -> Self {
        Self {
            radius,
            luminosity,
            temperature,
        }
    }
}

impl Material for HiResSunMaterial {}
