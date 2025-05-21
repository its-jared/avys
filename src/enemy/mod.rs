// TODO! Add enemies.

use bevy::prelude::*;

#[derive(Component)]
pub struct Hostile;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, _app: &mut App) {
        warn!("To be implemented!");
    }
}