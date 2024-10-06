use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    color::Color,
    math::Vec3,
    pbr::{PbrBundle, StandardMaterial},
    prelude::{
        default, BuildChildren, Camera3dBundle, Commands, Component, Cuboid, Mesh,
        OrthographicProjection, ResMut, Transform, TransformBundle,
    },
    render::camera::ScalingMode,
};
use bevy_rapier3d::prelude::{Collider, Damping, ExternalForce, LockedAxes, RigidBody};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, add_player);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
pub struct PlayerModel;

pub fn add_player(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera = create_camera();
    let model = create_model(meshes, materials);
    commands
        .spawn(TransformBundle::default())
        .insert(Player)
        .with_children(|parent| {
            parent.spawn(camera);
            parent
                .spawn((
                    PlayerModel,
                    model,
                    RigidBody::Dynamic,
                    Collider::cuboid(0.5, 0.5, 0.5),
                ))
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(Damping {
                    linear_damping: 5.0,
                    angular_damping: 0.9,
                })
                .insert(ExternalForce::default());
        });
}

fn create_model(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::srgb(0.8, 0.0, 0.0)),
        transform: Transform::from_xyz(1.5, 1.5, 1.5),
        ..default()
    }
}

fn create_camera() -> Camera3dBundle {
    Camera3dBundle {
        projection: OrthographicProjection {
            // 6 world units per window height.
            scaling_mode: ScalingMode::FixedVertical(6.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(2.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }
}
