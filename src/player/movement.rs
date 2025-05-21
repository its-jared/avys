use bevy::prelude::*;
use super::*;

pub fn handle_movement(
    mut player_q: Query<&mut Transform, With<Player>>,
    mut player_cam_q: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
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
        player.translation += direction.normalize() * speed * time.delta_secs();
        camera.translation = player.translation;
    }

    Ok(())
}