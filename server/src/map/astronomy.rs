//! a module contains utility functions for astronomy calculations.

use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

/// the mass of an astronomical object, unit in M☉
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct AstroMass(f32);

/// the radius of an astronomical object in local map, unit in R☉
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct AstroRadius(f32);

/// the luminosity of the star, in L☉
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Luminosity(f32);

/// the surface temperature of the star, in kelvin
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Temperature(f32);

impl AstroMass {
    pub fn new(mass: f32) -> Self {
        Self(mass)
    }
}

impl From<AstroMass> for f32 {
    fn from(mass: AstroMass) -> f32 {
        mass.0
    }
}

impl AstroRadius {
    pub fn new(radius: f32) -> Self {
        Self(radius)
    }
}

impl From<AstroRadius> for f32 {
    fn from(radius: AstroRadius) -> f32 {
        radius.0
    }
}

impl Luminosity {
    pub fn new(luminosity: f32) -> Self {
        Self(luminosity)
    }
}

impl From<Luminosity> for f32 {
    fn from(luminosity: Luminosity) -> f32 {
        luminosity.0
    }
}

impl Temperature {
    pub fn new(temperature: f32) -> Self {
        Self(temperature)
    }
}

impl From<Temperature> for f32 {
    fn from(temperature: Temperature) -> f32 {
        temperature.0
    }
}
