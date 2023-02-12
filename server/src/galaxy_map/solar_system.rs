use crate::{
    physics::collision::RapierCollider,
    utils::{marker::ServerEntityMarker, object_id::ObjectId},
};
use bevy::prelude::*;
use rand::prelude::*;
use rand_xoshiro::Xoroshiro64StarStar;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU16, Ordering};

#[derive(Component, Debug, Clone, Copy)]
pub struct SolarSystemMarker;

#[derive(Debug)]
pub struct SolarSystemGenerator {
    counter: AtomicU16,
    rng: Xoroshiro64StarStar,
}

impl SolarSystemGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            counter: AtomicU16::new(0),
            rng: SeedableRng::seed_from_u64(seed),
        }
    }

    pub fn generate(&self) -> SolarSystem {
        let x = ObjectId::new(
            0x01 << 24,
            self.counter.fetch_add(1, Ordering::Relaxed) as u32,
        );

        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::SolarSystemIdGenerator;

//     #[test]
//     fn object_id() {
//         let gen = SolarSystemIdGenerator::new(0);
//         let id1 = gen.spawn();
//         let id2 = gen.spawn();

//         assert_eq!(id1.prefix(), 0x01);
//         assert_eq!(id2.prefix(), 0x01);
//         assert_eq!(id1.full(), 0x01000000);

//         assert_ne!(id1, id2);
//     }
// }

/// a function to generate a transform component for solar system. a position is
/// generated as follows:
/// - uniformly randomly pick on point on a disk with radius `r`, which gives
///   `x` and `z`
/// - using normal distribution to pick a `y` value between `[-h, h]`, clamp is
///   required in this case.
/// - rotation is uniformly random in all direction.
fn rng_transform(rng: &mut impl Rng, r: f32, h: f32) -> Transform {
    todo!()
}

/// A solar system contains
/// - ObjectId: the id of the solar system
/// - Transform: bevy position component
/// - GlobalTransform[RT]: bevy position component
/// - RapierCollider[RT]: rapier collider handle for collision pipeline
/// - ServerEntityMarker[MK]: a marker to indicate this is a server entity
/// - SolarSystemMarker[MK]: a marker to indicate this is a solar system
pub struct SolarSystem(ObjectId, Transform);
