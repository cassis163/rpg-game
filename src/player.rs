use bevy::{
    app::{Plugin, PostUpdate, Startup},
    asset::Assets,
    color::Color,
    input::ButtonInput,
    math::Vec3,
    pbr::{PbrBundle, StandardMaterial},
    prelude::{
        default, BuildChildren, Camera, Camera3dBundle, Commands, Component, Cuboid, KeyCode, Mesh,
        OrthographicProjection, Parent, Query, Res, ResMut, Transform, TransformBundle, With,
        Without,
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

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostUpdate, update_players_movement);
    }
}

fn update_players_movement(
    mut query: Query<&mut ExternalForce, With<PlayerModel>>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    for mut impulse in query.iter_mut() {
        update_player_movement(&mut impulse, &key_input);
    }
}

fn update_player_movement(impulse: &mut ExternalForce, key_input: &ButtonInput<KeyCode>) {
    let direction = get_direction_vector(key_input);
    impulse.force = direction * 10.0;
}

fn get_direction_vector(key_input: &ButtonInput<KeyCode>) -> Vec3 {
    let mut direction = Vec3::ZERO;
    if key_input.pressed(KeyCode::KeyW) {
        direction -= Vec3::X;
    }
    if key_input.pressed(KeyCode::KeyS) {
        direction += Vec3::X;
    }
    if key_input.pressed(KeyCode::KeyA) {
        direction += Vec3::Z;
    }
    if key_input.pressed(KeyCode::KeyD) {
        direction -= Vec3::Z;
    }
    direction
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostUpdate, update_player_cameras);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerModel;

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
                .insert(Damping { linear_damping: 0.9, angular_damping: 0.9 })
                .insert(ExternalForce::default());
        });
}

fn update_player_cameras(
    mut camera_query: Query<(&mut Transform, &Parent), With<Camera>>,
    model_query: Query<(&Transform, &Parent), (With<PlayerModel>, Without<Camera>)>,
) {
    for (mut camera_transform, camera_parent) in camera_query.iter_mut() {
        for (model_transform, model_parent) in model_query.iter() {
            if camera_parent.get() == model_parent.get() {
                update_camera(&mut camera_transform, model_transform);
            }
        }
    }
}

fn update_camera(camera_transform: &mut Transform, player_transform: &Transform) {
    let player_position = player_transform.translation;
    let camera_position = camera_transform.translation;
    let new_camera_position = Vec3::new(player_position.x + 1.0, camera_position.y, player_position.z);
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
