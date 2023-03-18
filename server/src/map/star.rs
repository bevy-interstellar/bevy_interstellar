//! A module for all stars, stars is defined as non-moving astronomical object
//! with respect to reference of the solar system. There are different type of
//! stars
//! # components
//! ## internal properties
//! - AstroMass
//! - AstroRadius
//! - Luminosity
//! - Temperature: the surface temperature
//! - StarCategory
//! ## external properties
//! - Oid: from internal property
//! - Transform: local coordinate
//! ## constructed at runtime
//! - GlobalTransform

use crate::utils::oid::Oid;
use bevy::prelude::{Commands, Component, Entity, GlobalTransform, Transform};
use serde::{Deserialize, Serialize};

pub use super::astronomy::{AstroMass, AstroRadius, Luminosity, Temperature};

/// the category of the star
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StarCategory {
    /// a main sequence star with proton–proton chain fusion reaction in its
    /// core
    MainSeqPp,
    /// a main sequence star with carbon–nitrogen–oxygen cycle fusion reaction
    /// in its core
    MainSeqCno,
    /// a giant star whose luminosity is below 1380 L☉
    Giant,
    /// a giant star whose luminosity is between 1380 L☉ and 138000 L☉
    SuperGiant,
    /// a giant star whose luminosity is above 138000 L☉
    HyperGiant,
    /// a white dwarf is a compact star whose mass is below Chandrasekhar limit
    /// (1.4 M☉)
    WhiteDwarf,
    /// a neutron stars are compact stars whose mass is between Chandrasekhar
    /// limit (1.4 M☉) and Tolman–Oppenheimer–Volkoff limit (2.2 M☉)
    NeutronStar,
    /// (imaginary) a quark star is a compact stars whose mass is between
    /// Tolman–Oppenheimer–Volkoff limit (2.2 M☉) and 3.0 M☉
    QuarkStar,
    /// a black hole is a compact star whose mass is above 3 M☉
    BlackHole,
}

impl StarCategory {
    /// a main sequence star is any star that has a hot, dense core which fuses
    /// hydrogen into helium to produce energy
    pub fn main_sequence(&self) -> bool {
        match self {
            &Self::MainSeqPp | &Self::MainSeqCno => true,
            _ => false,
        }
    }

    /// giant stars are stars with substantially larger radius and luminosity
    /// than main-sequence stars of the same surface temperature.
    pub fn giant(&self) -> bool {
        match self {
            &Self::Giant | &Self::SuperGiant | &Self::HyperGiant => true,
            _ => false,
        }
    }

    /// a degenerate star is a compact star that is not a black hole
    pub fn degenerate(&self) -> bool {
        match self {
            &Self::WhiteDwarf | &Self::NeutronStar | &Self::QuarkStar => true,
            _ => false,
        }
    }

    /// a compact stars is the remnants of main sequence star, endpoints of
    /// stellar evolution.
    pub fn compact(&self) -> bool {
        match self {
            &Self::BlackHole => true,
            x if x.degenerate() => true,
            _ => false,
        }
    }
}

/// a star is a non-moving astronomical object with respect to solar system
/// reference. this is the object-oriented representation of the star, used for
/// generation & serialization.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct StarObject {
    pub id: Oid,

    pub mass: AstroMass,
    pub radius: AstroRadius,
    pub luminosity: Luminosity,
    pub temperature: Temperature,
    pub category: StarCategory,

    pub transform: Transform,
}

impl Default for StarObject {
    fn default() -> Self {
        Self {
            id: Default::default(),
            mass: AstroMass::new(0.0),
            radius: AstroRadius::new(0.0),
            luminosity: Luminosity::new(0.0),
            temperature: Temperature::new(0.0),
            category: StarCategory::MainSeqPp,
            transform: Default::default(),
        }
    }
}

impl StarObject {
    pub fn spawn(&self, commands: &mut Commands) -> Entity {
        commands
            .spawn_empty()
            .insert(self.id)
            .insert(self.mass)
            .insert(self.radius)
            .insert(self.luminosity)
            .insert(self.temperature)
            .insert(self.category)
            .insert(self.transform)
            .insert(GlobalTransform::default())
            .id()
    }
}
