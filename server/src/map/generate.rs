pub enum GalaxyShape {
    PARTIAL,
    DISC,
}

/// the parameter for random generating a map
pub struct MapGenArgs {
    seed: u64,
    galaxy_shape: GalaxyShape,
    galaxy_size: u32,
}
