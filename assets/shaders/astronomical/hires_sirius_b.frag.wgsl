#import noise::prelude
#import astro::common

struct HiResSiriusBMaterial {
    radius: f32,
    luminosity: f32,
    temperature: f32,
};
@group(1) @binding(0)
var<uniform> material: HiResSiriusBMaterial;

struct FragmentIn {
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) world_tangent: vec4<f32>,
}

@fragment
fn fragment(frag: FragmentIn) -> @location(0) vec4<f32> {
    let color = astro_kelvin_to_rgb(material.temperature);

    return vec4(color, 1.0);
}
