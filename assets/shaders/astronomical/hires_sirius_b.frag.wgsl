#import bevy_pbr::mesh_view_bindings
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
    let seed = material.radius * 1024.0;

    let color = astro_kelvin_to_rgb(material.temperature);
    let distance = distance(frag.world_position.xyz, view.world_position) * 0.01;
    let noise = astro_surface_noise(6.0 * frag.world_position.xyz / material.radius, globals.time * 0.1 + seed);

    return vec4(color * (0.5 + noise * 0.5) * HIRES_LUMINOSITY_FACTOR, 1.0);
}
