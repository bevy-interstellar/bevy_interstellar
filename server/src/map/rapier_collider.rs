use bevy::prelude::*;
use once_cell::unsync::Lazy;
use rapier3d::prelude::*;

//////////////////////// Component ////////////////////////

/// a handle for collision detection, used at L3 map.
///
/// should always reconstruct on reload
#[derive(Component, Debug, Copy, Clone)]
pub struct RapierCollider(ColliderHandle);

//////////////////////// Resource ////////////////////////

/// the engine for collision detection, used at L3 map.
///
/// should always reconstruct on reload
#[derive(Default)]
pub struct RapierCollisionEngine {
    pipeline: CollisionPipeline,

    colliders: ColliderSet,
    narrow_phase: NarrowPhase,
    broad_phase: BroadPhase,
}

// TODO: use collision group & user data
impl RapierCollisionEngine {
    /// create a empty collision engine.
    pub fn new() -> Self {
        Self::default()
    }

    /// pre-condition: collider.user_date() the lower 64 bits must be
    /// entity bits.
    fn spawn(&mut self, collider: Collider) -> RapierCollider {
        let user_data = collider.user_data;
        let handle = RapierCollider(self.colliders.insert(collider));
        info!("[physics] spawn handle: {:?}, {:?}", user_data, handle);
        handle
    }

    /// spawn a RapierCollider component for solar system
    pub fn spawn_solar_system(&mut self, e: Entity, trans: Vec3, r: f32) -> RapierCollider {
        let user_data = e.to_bits() as u128;

        let collider = ColliderBuilder::ball(r)
            .sensor(true)
            .active_collision_types(ActiveCollisionTypes::FIXED_FIXED)
            .user_data(user_data)
            .translation(trans.into())
            .build();
        self.spawn(collider)
    }

    /// remove an object from the engine
    pub fn remove(&mut self, handle: RapierCollider) {
        static mut ISLAND_MANAGER: Lazy<IslandManager> = Lazy::new(IslandManager::default);
        static mut RIGID_BODIES: Lazy<RigidBodySet> = Lazy::new(RigidBodySet::default);

        info!("[physics] remove handle: {:?}", handle);

        // this is safe because no operation is done for `ISLAND_MANAGER` and
        // `RIGID_BODIES`
        unsafe {
            self.colliders
                .remove(handle.0, &mut ISLAND_MANAGER, &mut RIGID_BODIES, false);
        }
    }

    /// update the next position for a entity with collision handle. please only
    /// call this function if the collider indeed move, because it will invalid
    /// the collider cache in the engine. this method can be used for ship
    /// movement for example.
    ///
    /// no effect if the handle is invalid
    pub fn update_transform(&mut self, handle: RapierCollider, dest: Vec3) {
        if let Some(collider) = self.colliders.get_mut(handle.0) {
            collider.set_translation(dest.into());
        } else {
            error!("[physics] invalid handle {:?}", handle);
        }
    }

    /// run one step of collision simulation
    pub fn step(&mut self) {
        static mut RIGID_BODIES: Lazy<RigidBodySet> = Lazy::new(RigidBodySet::default);

        // this is safe because no operation is done for `RIGID_BODIES`
        unsafe {
            self.pipeline.step(
                1000.0,
                &mut self.broad_phase,
                &mut self.narrow_phase,
                &mut RIGID_BODIES,
                &mut self.colliders,
                None,
                &(),
                &(),
            )
        }
    }

    /// get all intersections between game objects
    pub fn intersections(&self) -> impl Iterator<Item = (Entity, Entity)> + '_ {
        fn to_entity(colliders: &ColliderSet, handle: ColliderHandle) -> Entity {
            if let Some(collider) = colliders.get(handle) {
                let entity_bits = collider.user_data as u64;
                Entity::from_bits(entity_bits)
            } else {
                // this is invalid entity id.
                error!("[physics] invalid handle {:?}", handle);
                Entity::from_raw(0)
            }
        }

        self.narrow_phase
            .intersection_graph()
            .interactions_with_endpoints()
            .filter(|pair| pair.2.intersecting)
            .map(|pair| {
                (
                    to_entity(&self.colliders, pair.0),
                    to_entity(&self.colliders, pair.1),
                )
            })
    }
}
