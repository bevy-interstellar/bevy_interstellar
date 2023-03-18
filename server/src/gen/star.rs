use super::Generative;
use crate::map::star::{AstroMass, AstroRadius, Luminosity, StarCategory, StarObject, Temperature};
use crate::utils::oid::Oid;
use rand::Rng;
use rand_distr::{Distribution, Standard};

/// the random distribution based on initial mass function, with assumption that
/// the initial mass function follows the power law (i.e. x^a) and stars' mass
/// are between 0.1 and 300.
struct InitialMassDistribution {
    /// `a` is the exponent in the initial mass function f(x) = x^a
    a: f32,
    /// `c` is the scaling factor for random distribution
    c: f32,
}

impl InitialMassDistribution {
    /// create a new distribution
    fn new(a: f32) -> Self {
        let c = -(0.1_f32.powf(a + 1.) / (a + 1.)) + (300_f32.powf(a + 1.) / (a + 1.));
        Self { a: a, c: c }
    }

    /// the inverse of cumulative distribution function, used for random value
    /// generation.
    fn inv_cdf(&self, y: f32) -> f32 {
        let a = self.a;
        let c = self.c;
        (10.0_f32.powf(-a - 1.) + a * c * y + c * y).powf(1. / (a + 1.))
    }
}

#[cfg(test)]
mod tests {
    use super::InitialMassDistribution;
    use float_cmp::assert_approx_eq;

    #[test]
    fn initial_mass_distribution() {
        let distr = InitialMassDistribution::new(-1.3);
        let upls = 2_i32.pow(7);

        assert_approx_eq!(f32, distr.inv_cdf(0.0), 0.1, ulps = upls);
        assert_approx_eq!(f32, distr.inv_cdf(1.0), 300.0, ulps = upls);
        assert_approx_eq!(f32, distr.inv_cdf(0.5484742), 1.0, ulps = upls);
        assert_approx_eq!(f32, distr.inv_cdf(0.8233625), 10.0, ulps = upls);
        assert_approx_eq!(f32, distr.inv_cdf(0.9611330), 100.0, ulps = upls);
    }
}

impl Distribution<f32> for InitialMassDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f32 {
        let val = rng.sample(Standard);
        self.inv_cdf(val)
    }
}

impl Generative for StarObject {
    fn new_from_rng(rng: &mut impl Rng) -> Self {
        let mut star = StarObject::default();

        // this a value is not realistic, however, this allow us to have a higher
        // probability to generate larger stars.
        let initial_mass_distr = InitialMassDistribution::new(-1.3);
        let initial_mass = initial_mass_distr.sample(rng);

        // life = (M / MO)^{-2.5} in solar life time, 10 * 1e9 year,
        // giant life = 1/10 of life
        // compact star

        // TODO: use life & other factor the determine which one to call
        new_main_sequence(initial_mass, &mut star);

        star.id = Oid::v5_from_object(&(
            star.mass,
            star.radius,
            star.luminosity,
            star.temperature,
            star.category,
        ));

        star
    }
}
/// assign mass, luminosity, temperature and category to `out`
fn new_main_sequence(initial_mass: f32, out: &mut StarObject) {
    let mass = initial_mass;
    out.mass = AstroMass::new(mass);

    let radius = mass.powf(0.8);
    out.radius = AstroRadius::new(radius);

    let luminosity = mass.powf(3.5);
    out.luminosity = Luminosity::new(luminosity);

    // formula: L = R^2 * T^4, and use sun surface temperature as 5778 K
    let temperature = (luminosity.powf(0.25) / radius.powf(0.5)) * 5778.0;
    out.temperature = Temperature::new(temperature);

    let category = if mass < 1.5 {
        StarCategory::MainSeqPp
    } else {
        StarCategory::MainSeqCno
    };
    out.category = category;
}

/// pre-condition: 0.2 < initial_mass < 300.0
fn _new_giant(_initial_mass: f32) {
    todo!()
}
