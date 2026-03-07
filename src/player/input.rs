use bevy::prelude::*;

use crate::{entity::movement::MovementStats, player::Player};

pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut MovementStats, With<Player>>,
) {
    for mut movement_stats in q.iter_mut() {
        let mut dir = Vec3::ZERO;

        if keys.pressed(KeyCode::ArrowUp)    { dir += Vec3::Y; }
        if keys.pressed(KeyCode::ArrowDown)  { dir -= Vec3::Y; }
        if keys.pressed(KeyCode::ArrowRight) { dir += Vec3::X; }
        if keys.pressed(KeyCode::ArrowLeft)  { dir -= Vec3::X; }

        movement_stats.direction = dir;
    }
}