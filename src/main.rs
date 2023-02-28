use bevy::prelude::*;
use server::utils::oid::*;

fn main() {
    App::new()
        // external plugins
        .add_plugins(DefaultPlugins)
        // resources
        .insert_resource(OIdTable::new())
        // events
        // system
        // .add_startup_system(initialize_galaxy_map_system)
        .add_system_to_stage(CoreStage::PostUpdate, oid_table_update_system)
        .run();
}
