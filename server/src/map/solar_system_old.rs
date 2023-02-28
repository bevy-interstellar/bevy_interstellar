/// A solar system contains
/// - ObjectId: random generated or loaded
/// - Transform: random generated or loaded
/// - HillSphere: computed based on the stars in the system
/// - GlobalTransform: initialized by bevy
/// - RapierCollider: initialized when instantiate the entity
/// - ServerEntityMarker: marker
/// - SolarSystemMarker: marker
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SolarSystem {
    id: ObjectId,
    transform: Transform,
    radius: HillSphere,
}

impl SolarSystem {
    pub fn instantiate(&self, commands: &mut Commands, rapier_engine: &mut RapierCollisionEngine) {
        info!(
            "[galaxy_map] instantiate solar system {:?} at {:?}",
            self.id, self.transform.translation
        );

        let mut entity = commands.spawn((
            self.id,
            self.transform,
            self.radius,
            GlobalTransform::default(),
            ServerEntityMarker,
            SolarSystemMarker,
        ));

        let collider = rapier_engine.spawn_solar_system(
            entity.id(),
            self.transform.translation,
            self.radius.0,
        );

        entity.insert(collider);
    }
}

#[derive(Debug)]
pub struct SolarSystemGenerator {
    cnt: u16,
    rng: Xoroshiro64StarStar,
}

impl SolarSystemGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            cnt: 0,
            rng: SeedableRng::seed_from_u64(seed),
        }
    }

    /// get a unique id
    fn cnt_fetch_inc(&mut self) -> u32 {
        let i = self.cnt;
        self.cnt += 1;
        i as u32
    }

    #[inline(always)]
    fn rng_uniform(&mut self) -> f32 {
        <Standard as Distribution<f32>>::sample(&Standard, &mut self.rng)
    }

    fn generate_transform(&mut self, r: f32, h: f32) -> Transform {
        // compute translate
        let r_rng = r * self.rng_uniform().sqrt();
        let t_rng = 2.0 * PI * self.rng_uniform();
        let x = r_rng * t_rng.cos();
        let z = r_rng * t_rng.sin();
        let y = 0.0; // TODO: use normal distribution

        // compute rotation
        // TODO
        Transform::from_xyz(x, y, z)
    }

    pub fn generate(&mut self) -> SolarSystem {
        let id = ObjectId::new(0x01, self.cnt_fetch_inc());
        let transform = self.generate_transform(320.0, 3.0);
        let radius = HillSphere(1.0);

        SolarSystem {
            id: id,
            transform: transform,
            radius: radius,
        }
    }
}
