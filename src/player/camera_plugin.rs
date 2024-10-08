use bevy::{
    app::{Plugin, PostUpdate},
    math::Vec3,
    prelude::{Camera, Entity, Parent, Query, Transform, With, Without},
};
use crate::player::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostUpdate, update_player_cameras);
    }
}

fn update_player_cameras(
    mut camera_query: Query<(&mut Transform, &Parent), With<Camera>>,
    model_query: Query<(Entity, &Transform), (With<Player>, Without<Camera>)>,
) {
    for (mut camera_transform, camera_parent) in camera_query.iter_mut() {
        for (player_entity, transform) in model_query.iter() {
            if camera_parent.get() == player_entity {
                update_camera(&mut camera_transform, transform);
            }
        }
    }
}

fn update_camera(camera_transform: &mut Transform, player_transform: &Transform) {
    let local_position = Vec3::new(4.0, 10.0, 0.0);
    let world_position = player_transform.translation + local_position;
    camera_transform.translation = world_position - player_transform.translation;
}
