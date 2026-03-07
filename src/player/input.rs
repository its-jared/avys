use bevy::prelude::*;

use crate::{entity::{dash::Dashing, movement::MovementStats}, player::Player};

pub fn handle_input(
    mut c: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<(Entity, &mut MovementStats), With<Player>>,
) {
    for (entity, mut movement_stats) in q.iter_mut() {
        let mut dir = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) { dir += Vec3::Y; }
        if keys.pressed(KeyCode::KeyS) { dir -= Vec3::Y; }
        if keys.pressed(KeyCode::KeyD) { dir += Vec3::X; }
        if keys.pressed(KeyCode::KeyA) { dir -= Vec3::X; }

        if keys.just_pressed(KeyCode::KeyQ) {
            c.entity(entity).insert(Dashing);
        }

        movement_stats.direction = dir;
    }
}