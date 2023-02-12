use std::sync::atomic::{AtomicU16, Ordering};
use utils::{id::ObjectIdGenerator, prelude::*};

pub struct SolarSystemIdGenerator {
    counter: AtomicU16,
}

impl Default for SolarSystemIdGenerator {
    fn default() -> Self {
        return Self::new();
    }
}

impl ObjectIdGenerator for SolarSystemIdGenerator {
    fn prefix() -> u8 {
        0x1
    }

    fn spawn(&self) -> ObjectId {
        let prefix = (Self::prefix() as u32) << 24;
        let base = self.counter.fetch_add(1, Ordering::Relaxed) as u32;
        ObjectId::new(prefix | base)
    }
}

impl SolarSystemIdGenerator {
    pub const fn new() -> Self {
        Self {
            counter: AtomicU16::new(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solar_system::id::SolarSystemIdGenerator;
    use utils::id::ObjectIdGenerator;

    #[test]
    fn object_id() {
        let gen = SolarSystemIdGenerator::new();
        let id1 = gen.spawn();
        let id2 = gen.spawn();

        assert_eq!(id1.prefix(), 0x01);
        assert_eq!(id2.prefix(), 0x01);
        assert_eq!(id1.full(), 0x01000000);

        assert_ne!(id1, id2);
    }
}
