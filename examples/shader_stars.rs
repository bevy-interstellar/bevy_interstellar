use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;

use client::map::star::sirius_b::HiResSiriusBMaterial;
use client::utils::sphere::CubeSphere;
use client::ClientPlugin;

#[derive(Component)]
struct Movable;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
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
    let mesh: Mesh = CubeSphere {
        radius: 1.0,
        resolution: 32,
    }
    .into();

    commands.spawn(MaterialMeshBundle::<HiResSiriusBMaterial> {
        mesh: meshes.add(mesh.clone()),
        material: materials.add(HiResSiriusBMaterial {
            radius: 1.711,
            luminosity: 25.4,
            temperature: 9940.0,
        }),
        transform: Transform::from_xyz(-1.5, 0.0, 0.0),
        ..default()
    });

    commands.spawn(MaterialMeshBundle::<HiResSiriusBMaterial> {
        mesh: meshes.add(mesh.clone()),
        material: materials.add(HiResSiriusBMaterial {
            radius: 1.0,
            luminosity: 1.0,
            temperature: 5778.0,
        }),
        transform: Transform::from_xyz(1.5, 0.0, 0.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            // from 1 AU
            transform: Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                hdr: true,
                ..default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: std::f32::consts::PI / 8.,
                ..default()
            }),
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
        Movable,
    ));
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
            transform.translation.z -= time.delta_seconds() * 50.0;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.z += time.delta_seconds() * 50.0;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= time.delta_seconds() * 50.0;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += time.delta_seconds() * 50.0;
        }
        if input.pressed(KeyCode::Space) {
            transform.translation.y += time.delta_seconds() * 50.0;
        }
        if input.pressed(KeyCode::LShift) {
            transform.translation.y -= time.delta_seconds() * 50.0;
        }
    }
}
