use bevy::prelude::*;
use client::ClientPlugin;
use server::utils::oid::*;

fn main() {
    App::new()
        // external plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin)
        .run();
}
