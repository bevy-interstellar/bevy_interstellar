pub mod map;
pub mod utils;

use bevy::prelude::Plugin;
use utils::shaders::system_init_shader;

pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(system_init_shader);
    }
}
