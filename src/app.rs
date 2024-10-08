use bevy::prelude::*;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use crate::{
    npc::npc_plugin::NpcPlugin, player::{
        camera_plugin::CameraPlugin, movement_plugin::PlayerMovementPlugin,
        player_plugin::PlayerPlugin,
        actions_plugin::ActionsPlugin,
    }, scene::ScenePlugin
};

pub fn launch_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ScenePlugin)
        .add_plugins(NpcPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerMovementPlugin)
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
        .add_plugins(ActionsPlugin{blocking: true})
        .run();
}
