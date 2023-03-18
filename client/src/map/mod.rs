pub mod star;

use bevy::prelude::{Entity, Handle, Mesh, Resource};

#[derive(Resource, Debug, Clone, Copy)]
pub enum MapView {
    L3,
    L2(Entity),
}

#[derive(Resource, Debug)]
pub struct HiResSphere(Handle<Mesh>);

#[derive(Resource, Debug)]
pub struct LoResSphere(Handle<Mesh>);
