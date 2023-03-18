#![allow(mixed_script_confusables)]

pub mod fleet;
pub mod gen;
pub mod map;
pub mod utils;

use bevy::prelude::{App, Plugin};

struct BevyInterstellarServerPlugin;

impl Plugin for BevyInterstellarServerPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
