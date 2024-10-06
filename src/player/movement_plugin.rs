use bevy::{
    app::{Plugin, PostUpdate},
    input::ButtonInput,
    math::Vec3, prelude::{KeyCode, Query, Res, With},
};
use bevy_rapier3d::prelude::ExternalForce;

use super::player_plugin::PlayerModel;

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
