use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::*;

pub fn handle_movement(
    mut player_q: Query<(&mut KinematicCharacterController, &Transform), With<Player>>,
    mut player_cam_q: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) -> Result<(), BevyError> {
    let mut player = player_q.single_mut()?;
    let mut camera = player_cam_q.single_mut()?;
    let mut direction = Vec2::ZERO; 
    let speed = 200.0;

    camera.translation = player.1.translation;

    if keys.pressed(KeyCode::KeyW) { direction += Vec2::Y; }
    if keys.pressed(KeyCode::KeyS) { direction -= Vec2::Y; }

    if keys.pressed(KeyCode::KeyA) { direction -= Vec2::X; }
    if keys.pressed(KeyCode::KeyD) { direction += Vec2::X; }

    if direction != Vec2::ZERO {
        player.0.translation = Some(direction.normalize() * speed * time.delta_secs());
    }

    Ok(())
}