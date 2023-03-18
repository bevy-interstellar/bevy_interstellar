use bevy::prelude::*;
use client::map::star::*;
use server::utils::oid::*;

fn main() {
    App::new()
        // external plugins
        .add_plugins(DefaultPlugins)
        .add_startup_system(init_system)
        .run();
}

fn init_system(mut commands: Commands)
