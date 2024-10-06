use bevy::prelude::*;

use crate::{player::{CameraPlugin, PlayerMovementPlugin, PlayerPlugin}, scene::ScenePlugin};

pub fn launch_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerMovementPlugin)
        .run();
}
