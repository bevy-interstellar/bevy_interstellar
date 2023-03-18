//! A `Solar System` is an stellar system in the main map,
//! # components
//! ## internal properties
//! - ContainStars
//! - ContainPlanets
//! - ContainFleets
//!
//! ## external properties
//! ## construed at runtime
//!
//! - ObjectId
//! - Transform (and GlobalTransform): the translation & rotation in the L3 map
//! - AstroMass: the sum of `AstroMass` in `ContainsStars` and `ContainsPlanets`
//! - HillSphere: the radius of the solar system, computed based on the
//!   `AstroMass`
//! - ContainsStars: list of Entity for stars
//! - ContainsPlanets: list of Entity for planets
//! - ContainsFleets: list of Entity for fleets
//! - RapierCollider: the handle for rapier physics engine
//! - SolarSystemMarker: a marker component to indicate this a a solar system
//! - ServerEntityMarker: a a marker component to indicate this is a server
//!   entity

use super::astronomy::{AstroMass, AstroRadius};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// an approximation of the hill sphere radius, use equation
/// $$a \times \sqrt[3]{\frac{m}{3M}}$$ and pick a, M as constant.
pub fn solar_system_radius(mass: f32) -> f32 {
    mass.powf(1. / 3.)
}

/// a marker for solar system
#[derive(Component, Debug, Clone, Copy)]
pub struct SolarSystemMarker;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarSystemSerde {
    // id: ObjectId,
    transform: Transform,
    mass: AstroMass,
    radius: AstroRadius,
}

impl SolarSystemSerde {
    // transform
    // compute translate
    // let r_rng = r * self.rng_uniform().sqrt();
    // let t_rng = 2.0 * PI * self.rng_uniform();
    // let x = r_rng * t_rng.cos();
    // let z = r_rng * t_rng.sin();
    // let y = 0.0; // TODO: use normal distribution
}
