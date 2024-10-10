use bevy::{
    app::{Plugin, PostUpdate},
    input::ButtonInput,
    math::Vec3, prelude::{KeyCode, Query, Res, With},
};
use bevy::prelude::{Event, EventReader, Local};
use bevy_rapier3d::prelude::ExternalForce;
use crate::player::actions_plugin::ToggleInputEvent;
use crate::player::player::Player;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostUpdate, update_players_movement_constructor());
    }
}

#[derive(Default)]
struct PlayerMovementState {
    can_move: bool,
}

fn update_players_movement_constructor() -> impl FnMut(Query<&mut ExternalForce, With<Player>>, Res<ButtonInput<KeyCode>>, EventReader<ToggleInputEvent>) {
    let mut pms = PlayerMovementState { can_move: true };
    move |mut query, key_input, mut toggle_input_event| {
        for tie in toggle_input_event.read() {
            pms.can_move = !tie.is_toggled;
        }

        if !pms.can_move {
            return;
        }

        for mut impulse in query.iter_mut() {
            update_player_movement(&mut impulse, &key_input);
        }
    }
}

fn update_players_movement(
    mut query: Query<&mut ExternalForce, With<Player>>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    for mut impulse in query.iter_mut() {
        update_player_movement(&mut impulse, &key_input);
    }
}

fn update_player_movement(impulse: &mut ExternalForce, key_input: &ButtonInput<KeyCode>) {
    let direction = get_direction_vector(key_input);
    impulse.force = direction * 15.0;
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