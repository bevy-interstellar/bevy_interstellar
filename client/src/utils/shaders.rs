use bevy::asset::HandleId;
use bevy::log::*;
use bevy::prelude::*;

macro_rules! shader {
    ($uri:literal) => {
        Shader::from_wgsl(include_str!(concat!("../../../assets/shaders/", $uri)))
    };
}

pub fn system_init_shader(mut shaders: ResMut<Assets<Shader>>) {
    for shader in [
        shader!("noise/common.wgsl"),
        shader!("noise/perlin_vec2f.wgsl"),
        shader!("noise/perlin_vec3f.wgsl"),
        shader!("noise/perlin_vec4f.wgsl"),
        shader!("noise/simplex_vec2f.wgsl"),
        shader!("noise/simplex_vec3f.wgsl"),
        shader!("noise/simplex_vec4f.wgsl"),
        shader!("noise/worley_vec2f.wgsl"),
        shader!("noise/worley_vec3f.wgsl"),
        shader!("noise/prelude.wgsl"),
        shader!("astronomical/common.wgsl"),
    ] {
        let handle = HandleId::random::<Shader>();
        shaders.set_untracked(handle, shader);
    }
}
