use bevy::prelude::*;
use crate::level::block;

use super::*;

pub fn handle_movement(
    mut player_q: Query<&mut Transform, With<Player>>,
    mut player_cam_q: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    level: Res<level::Level>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) -> Result<(), BevyError> {
    let mut player = player_q.single_mut()?;
    let mut camera = player_cam_q.single_mut()?;
    let mut direction = Vec3::ZERO; 
    let speed = 200.0;

    if keys.pressed(KeyCode::KeyW) { direction += Vec3::Y; }
    if keys.pressed(KeyCode::KeyS) { direction -= Vec3::Y; }

    if keys.pressed(KeyCode::KeyA) { direction -= Vec3::X; }
    if keys.pressed(KeyCode::KeyD) { direction += Vec3::X; }

    if direction != Vec3::ZERO {
        let going_to = direction.normalize() * speed * time.delta_secs();
        let level_pos = level::world_to_level_pos(Vec2::new((player.translation + going_to).x, (player.translation + going_to).y));
        let mut allow_move = false;

        if let Some(floor) = level.get_block(level_pos, block::BlockLayer::Floor) {
            allow_move = !level.block_registery.get(floor).unwrap().solid;
        }
        if let Some(wall) = level.get_block(level_pos, block::BlockLayer::Wall) {
            allow_move = !level.block_registery.get(wall).unwrap().solid;
        }

        if allow_move {
            player.translation += going_to;
            camera.translation = player.translation;
        }
    }

    Ok(())
}