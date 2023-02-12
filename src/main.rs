use bevy::prelude::*;
use server::utils::object_id::*;

fn main() {
    App::new()
        // external plugins
        .add_plugins(DefaultPlugins)
        // resources
        .insert_resource(ObjectIdTable::new())
        // events
        // system
        .add_system_to_stage(CoreStage::PostUpdate, object_id_table_update_system)
        .run();
}
