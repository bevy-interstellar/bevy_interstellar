use rand::Rng;

pub mod star;

pub trait Generative {
    fn new_from_rng(rng: &mut impl Rng) -> Self;
}
