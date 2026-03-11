use bevy::prelude::*;

pub mod dash;
pub mod movement;

#[derive(Component)]
pub struct Entity;

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    movement::handle_movement,
                    dash::on_dash_start,
                    dash::handle_dash,
            ).chain());
    }
}