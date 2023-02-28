//! a module for generating, saving or loading the map, it also handles all the
//! movement or change in the map.
//!
//! the map consist of 2 layer. the normal space (where the maximum speed limit
//! is light) and hyper space (where the minimum speed limit is light).
//! no interaction can occur between two layers other than hyper space
//! fortification.
//! - in normal space, fleet can interact with fleet or solar system. note that
//!   solar system does not move, so no solar system-solar system interaction
//!   possible.
//! - in hyper space, interaction only happens between fleet and hyper space
//!   fortification. fleet-fleet interaction or fortification-fortification
//!   interaction is ignored.

pub mod astronomy;
pub mod generate;
pub mod rapier_collider;
pub mod solar_system;
pub mod star;

use bevy::prelude::{Commands, ResMut};

// use self::solar_system::SolarSystemGenerator;

// pub mod solar_system;

// pub fn initialize_galaxy_map_system(
//     mut commands: Commands,
//     mut collision_engine: ResMut<RapierCollisionEngine>,
// ) {
//     let mut generator = SolarSystemGenerator::new(92808428);

//     for _ in 0..4096 {
//         let solar_system = generator.generate();
//         solar_system.instantiate(&mut commands, collision_engine.as_mut());
//     }
// }
