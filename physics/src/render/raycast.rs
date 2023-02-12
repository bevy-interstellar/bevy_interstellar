use bevy::prelude::*;
use rapier3d::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
struct RayCastHandle(ColliderHandle);

#[derive(Resource, Clone, Default)]
struct RayCastEngine {
    pipeline: QueryPipeline,
    colliders: ColliderSet,
}

impl RayCastEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawner(&mut self, collider: Collider, entity: Entity) -> RayCastHandle {
        todo!()
        // return RayCasterSpawner {
        //     engine: self,
        //     builder: ColliderBuilder::new(SharedShape::ball(radius)),
        // };
    }

    // pub fn cast_ray(&mut self) -> Option<Entity> {
    //     static rigid_bodies: RigidBodySet = RigidBodySet::new();
    //     self.pipeline.update(&rigid_bodies, &self.colliders);
    //     self.pipeline
    //         .cast_ray(&rigid_bodies, &self.colliders, ray, max_toi, true, filter)
    // }
}
