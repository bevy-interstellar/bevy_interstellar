#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) tangent: vec4<f32>,
};

struct Fragment {
    @builtin(poistion) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) world_tangent: vec4<f32>,
}

@vertex
fn vertex(vert: Vertex) -> Fragment {
    var out: Fragment;
    var model = mesh.model;

    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vert.position, 1.0));
    out.clip_position = mesh_position_world_to_clip(out.world_position);
    out.world_normal = mesh_normal_local_to_world(vert.normal);
    out.uv = vert.uv;
    out.world_tangent = mesh_tangent_local_to_world(model, vert.tangent);

    return out;
}