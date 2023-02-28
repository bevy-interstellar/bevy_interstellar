//! A module for all stars, stars is defined as non-moving astronomical object
//! with respect to reference of the solar system. There are different type of
//! stars
//! - BlackHole
//! - DegenerateStar (QuarkStar, NeutronStar, WhiteDwarf, BlackDwarf, etc)
//! - MainSequenceStar (Blue Giant, RedDwarf, BrownDwarf, etc)
//! - GiantStar (HyperGiant, SubGiant, etc)
//!
//! a star include the following components
//! - Transform (and GlobalTransform): the translation & rotation in the L3 map
//! - AstroMass: the mass of star
//! - AstroRadius: the radius of star

use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

// is a source of light
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Luminary {
    luminosity: f32,
    temperature: f32,
}
