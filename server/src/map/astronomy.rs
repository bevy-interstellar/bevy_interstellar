//! a module contains utility functions for astronomy calculations.

use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

/// the mass of an astronomical object, unit in M☉
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AstroMass(f32);

/// the radius of an astronomical object in local map, unit in R☉
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AstroRadius(f32);

/// the luminosity of the star, in L☉
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Luminosity(f32);

/// the surface temperature of the star, in kelvin
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Temperature(f32);
