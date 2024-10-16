use bevy::prelude::Component;
use bevy::{
    asset::Assets,
    color::Color,
    pbr::{PbrBundle, StandardMaterial},
    prelude::{default, Commands, Cuboid, Entity, Mesh, ResMut, Transform},
};
use bevy_rapier3d::prelude::{Collider, Damping, ExternalForce, LockedAxes, RigidBody};
use std::collections::HashMap;
use crate::item::Item;

#[derive(Component)]
pub struct Character;

pub fn spawn_character_entity(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    color: Color,
    position: (f32, f32, f32),
) -> Entity {
    let model = PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(color),
        transform: Transform::from_xyz(position.0, position.1, position.2),
        ..default()
    };
    commands
        .spawn((
            Character,
            model,
            RigidBody::Dynamic,
            Collider::cuboid(0.5, 0.5, 0.5),
        ))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Damping {
            linear_damping: 5.0,
            angular_damping: 0.9,
        })
        .insert(ExternalForce::default())
        .id()
}

pub trait CharacterTrait {
    fn set_items(&mut self, new_items: HashMap<Item, i32>);
    fn add_item(&mut self, item: Item, amount: i32);
    fn remove_item(&mut self, item: Item, amount: i32) -> bool;
    fn get_items(&self) -> &HashMap<Item, i32>;
    fn print_self(&self);
}