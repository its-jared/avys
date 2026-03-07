use bevy::prelude::*;

pub mod movement;

#[derive(Component)]
pub struct Entity;

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement::handle_movement);
    }
}