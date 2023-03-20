use bevy::prelude::Material;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Copy, Default)]
#[uuid = "4291d443-7434-4be9-915a-e07880b657b9"]
pub struct HiResSiriusBMaterial {
    #[uniform(0)]
    pub radius: f32,
    #[uniform(0)]
    pub luminosity: f32,
    #[uniform(0)]
    pub temperature: f32,
}

impl Material for HiResSiriusBMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/astronomical/hires.vert.wgsl".into()
    }

    #[allow(unused_variables)]
    fn fragment_shader() -> ShaderRef {
        "shaders/astronomical/hires_sirius_b.frag.wgsl".into()
    }
}
