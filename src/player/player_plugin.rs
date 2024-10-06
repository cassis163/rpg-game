use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    color::Color,
    math::Vec3,
    pbr::StandardMaterial,
    prelude::{
        default, BuildChildren, Camera3dBundle, Commands, Component, Mesh, OrthographicProjection,
        ResMut, Transform,
    },
    render::camera::ScalingMode,
};

use crate::character::spawn_character_entity;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let character = spawn_character_entity(
        &mut commands,
        meshes,
        materials,
        Color::srgb(1.0, 0.0, 0.0),
        (0.0, 2.0, 0.0),
    );
    let camera = create_camera();
    commands
        .entity(character)
        .insert(Player)
        .with_children(|parent| {
            parent.spawn(camera);
        });
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
