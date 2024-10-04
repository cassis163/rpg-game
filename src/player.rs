use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    color::Color,
    math::Vec3,
    pbr::{PbrBundle, StandardMaterial},
    prelude::{
        default, BuildChildren, Camera, Camera3dBundle, Children, Commands, Component,
        Cuboid, Mesh, OrthographicProjection, Query, ResMut, Transform, TransformBundle, With,
    },
    render::camera::ScalingMode,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, add_player);
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, update_player_cameras);
    }
}

#[derive(Component)]
struct Player;

pub fn add_player(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera = create_camera();
    let model = create_model(meshes, materials);
    let player_entity = commands
        .spawn(TransformBundle::default())
        .insert(Player)
        .id();
    commands.entity(player_entity).with_children(|parent| {
        parent.spawn(camera);
        parent.spawn(model);
    });
}

fn update_player_cameras(
    player_query: Query<(&Children, &Transform), With<Player>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for (children, player_transform) in player_query.iter() {
        for &child in children.iter() {
            if let Ok(mut camera_transform) = camera_query.get_mut(child) {
                update_camera(&mut camera_transform, player_transform);
            }
        }
    }
}

fn update_camera(camera_transform: &mut Transform, player_transform: &Transform) {
    let player_position = player_transform.translation;
    let camera_position = camera_transform.translation;
    let new_camera_position = Vec3::new(player_position.x, camera_position.y, player_position.z);
    camera_transform.translation = new_camera_position;
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
