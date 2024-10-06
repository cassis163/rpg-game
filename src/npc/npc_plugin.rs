use bevy::app::{App, Plugin, Startup};

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_npcs);
    }
}

fn spawn_npcs() {

}

fn spawn_npc() {

}