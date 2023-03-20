use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::*};

use client::map::star::sirius_b::HiResSiriusBMaterial;
use client::utils::sphere::CubeSphere;
use client::ClientPlugin;

#[derive(Component)]
struct Movable;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(MaterialPlugin::<HiResSiriusBMaterial>::default())
        .add_startup_system(setup)
        .add_system(object_rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<HiResSiriusBMaterial>>,
) {
    let mesh: Mesh = CubeSphere::default().into();

    commands.spawn(MaterialMeshBundle::<HiResSiriusBMaterial> {
        mesh: meshes.add(mesh),
        material: materials.add(HiResSiriusBMaterial {
            radius: 1.711,
            luminosity: 25.4,
            temperature: 9940.0,
        }),
        ..default()
    });

    let camera_origin = commands.spawn((TransformBundle::default(), Movable)).id();

    let camera = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .id();

    commands.entity(camera_origin).add_child(camera);
}

fn object_rotate(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in &mut query {
        if input.pressed(KeyCode::Up) {
            transform.rotate_x(time.delta_seconds());
        }
        if input.pressed(KeyCode::Down) {
            transform.rotate_x(-time.delta_seconds());
        }
        if input.pressed(KeyCode::Left) {
            transform.rotate_y(time.delta_seconds());
        }
        if input.pressed(KeyCode::Right) {
            transform.rotate_y(-time.delta_seconds());
        }
        if input.pressed(KeyCode::W) {
            transform.translation.z += time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.z -= time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x += time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x -= time.delta_seconds();
        }
        if input.pressed(KeyCode::Space) {
            transform.translation.y += time.delta_seconds();
        }
        if input.pressed(KeyCode::LShift) {
            transform.translation.y -= time.delta_seconds();
        }
    }
}
