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
use crate::utils::oid::Oid;
use bevy::prelude::*;
use postcard;
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
    /// a main sequence star with triple-alpha process fusion reaction in its
    /// core
    MainSeqTα,
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
            &Self::MainSeqPp | &Self::MainSeqCno | &Self::MainSeqTα => true,
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

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct StarObject {
    id: Oid,
    transform: Transform,
    mass: AstroMass,
    radius: AstroRadius,
    luminosity: Luminosity,
    temperature: Temperature,
    category: StarCategory,
}

#[derive(Serialize, Deserialize)]
struct StarObjectLite(
    AstroMass,
    AstroRadius,
    Luminosity,
    Temperature,
    StarCategory,
);

impl Into<StarObject> for StarObjectLite {
    fn into(self) -> StarObject {
        let mut buf = [0_u8; 40];
        let data = postcard::to_slice(&(self.0, self.1, self.2, self.3, self.4), &mut buf).unwrap();

        StarObject {
            id: Oid::v5(&data),
            transform: Default::default(),
            mass: self.0,
            radius: self.1,
            luminosity: self.2,
            temperature: self.3,
            category: self.4,
        }
    }
}

impl StarObject {
    pub fn load(save_id: Oid) -> Self {
        todo!()
    }

    pub fn save(&self, save_id: Oid) {
        todo!()
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        let mut star: StarObject = main_seq::generate(rng).into();
        star
    }
}
