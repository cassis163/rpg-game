use bevy::{app::{Plugin, Startup}, asset::Assets, color::Color, pbr::{PbrBundle, PointLightBundle, StandardMaterial}, prelude::{default, Commands, Cuboid, Mesh, Meshable, Plane3d, ResMut, Transform}};
use bevy_rapier3d::prelude::Collider;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_scene);
    }
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane = PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(25.0, 25.0)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    };
    // Plane with collider
    commands.spawn((plane, Collider::cuboid(25.0, 0.1, 25.0)));
    
    // Cubes with colliders
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(1.5, 0.5, 1.5),
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5), // Collider for the cube
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(1.5, 0.5, -1.5),
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5), // Collider for the cube
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(-1.5, 0.5, 1.5),
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5), // Collider for the cube
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(-1.5, 0.5, -1.5),
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5), // Collider for the cube
    ));

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}
