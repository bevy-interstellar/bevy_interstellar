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

use super::astronomy::*;
use crate::utils::{oid::Oid, property::Property};
use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

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
            _ if self.degenerate() => true,
            _ => false,
        }
    }
}

/// the natural property of a star
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct StarProperty {
    mass: AstroMass,
    radius: AstroRadius,
    luminosity: Luminosity,
    temperature: Temperature,
    category: StarCategory,
}

impl Property for StarProperty {}

/// a star is a non-moving astronomical object with respect to solar system
/// reference. this is the object-oriented representation of the star, used for
/// generation & serialization.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct StarObject {
    id: Oid,
    property: StarProperty,
    transform: Transform,
}

impl StarObject {
    pub fn load(save: Oid) -> Self {
        todo!()
    }

    pub fn save(&self, save: Oid) {}

    /// pre-condition: 0.1 < initial_mass < 120.0
    fn main_sequence(initial_mass: f32) -> StarProperty {
        let mass = initial_mass;

        // from paper DOI 10.1007/BF00639097
        let category = if mass < 1.66 {
            StarCategory::MainSeqPp
        } else {
            StarCategory::MainSeqCno
        };

        let radius = if mass < 1.66 {
            0.89 * mass.powf(0.89)
        } else {
            1.01 * mass.powf(0.57)
        };

        let luminosity = if mass < 0.7 {
            0.20 * mass.powf(2.50)
        } else {
            1.15 * mass.powf(3.36)
        };

        // formula: L = R^2 * T^4, and use sun surface temperature as 5778 K
        let temperature = (luminosity.powf(0.25) / radius.powf(0.5)) * 5778.0;

        StarProperty {
            mass: AstroMass::new(mass),
            radius: AstroRadius::new(radius),
            luminosity: Luminosity::new(luminosity),
            temperature: Temperature::new(temperature),
            category: category,
        }
    }

    /// pre-condition: 0.2 < initial_mass < 300.0
    pub fn giant(initial_mass: f32) -> StarProperty {
        todo!()
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        let property = Self::main_sequence(1.0);

        // life = (M / MO)^{-2.5} in solar life time, 10 * 1e9 year,
        // giant life = 1/10 of life
        // compact star

        StarObject {
            id: property.id(),
            property: property,
            transform: Default::default(),
        }
    }
}
