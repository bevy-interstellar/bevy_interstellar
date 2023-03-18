use crate::map::HiResSphere;
use bevy::prelude::*;
use server::map::star::*;
use server::utils::sync::SyncTo;

use super::sun::HiResSunMaterial;

pub fn attach_graphics_system(
    mut commands: Commands,
    stars: Query<(
        Entity,
        &Transform,
        &AstroRadius,
        &Luminosity,
        &StarCategory,
        &Temperature,
    )>,
    mesh: Res<HiResSphere>,
    mut sun_materials: ResMut<Assets<HiResSunMaterial>>,
) {
    for (entity, transform, radius, luminosity, category, temperature) in stars.iter() {
        let eid = match category {
            x if x.main_sequence() => commands
                .spawn(MaterialMeshBundle::<HiResSunMaterial> {
                    material: sun_materials.add(HiResSunMaterial::new(
                        (*radius).into(),
                        (*luminosity).into(),
                        (*temperature).into(),
                    )),
                    mesh: mesh.0.clone(),
                    transform: *transform,
                    ..Default::default()
                })
                .id(),
            _ => panic!("undefined category"),
        };

        commands.entity(entity).insert(SyncTo::new(eid));
    }
}

pub fn detach_graphics_system(
    mut commands: Commands,
    stars: Query<Entity, (With<StarCategory>, With<SyncTo>)>,
    client_stars: Query<Entity, With<Handle<HiResSunMaterial>>>,
) {
    for entity in stars.iter() {
        commands.entity(entity).remove::<SyncTo>();
    }

    for entity in client_stars.iter() {
        commands.entity(entity).despawn();
    }
}
