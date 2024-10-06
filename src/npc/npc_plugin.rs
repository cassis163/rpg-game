use bevy::{app::{App, Plugin, Startup}, asset::Assets, color::Color, pbr::StandardMaterial, prelude::{Commands, Component, Mesh, ResMut}};

use crate::character::spawn_character_entity;
use crate::npc::npc::Npc;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_npcs);
    }
}

fn spawn_npcs(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_npc(commands, meshes, materials);
}

fn spawn_npc(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let character = spawn_character_entity(
        &mut commands,
        meshes,
        materials,
        Color::srgb(0.0, 0.0, 1.0),
        (0.0, 5.0, 2.0),
    );
    let client = reqwest::Client::new();
    // commands
    //     .entity(character)
    //     .insert(Npc::new("Hank", "Blacksmith", "Hank is a well respected blacksmith in the Kingdom of Veldora"));
}