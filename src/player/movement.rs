use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::gui::debug_ui::PlayerPosLabel;

use super::*;

pub fn handle_movement(
    mut player_q: Query<(&mut KinematicCharacterController, &Transform), With<Player>>,
    mut player_cam_q: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    mut label_q: Query<&mut Text, With<PlayerPosLabel>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) -> Result<(), BevyError> {
    let mut player = player_q.single_mut()?;
    let mut camera = player_cam_q.single_mut()?;
    let mut direction = Vec2::ZERO; 
    let mut label = label_q.single_mut()?;
    let speed = 200.0;

    camera.translation = player.1.translation;

    if keys.pressed(KeyCode::KeyW) { direction += Vec2::Y; }
    if keys.pressed(KeyCode::KeyS) { direction -= Vec2::Y; }

    if keys.pressed(KeyCode::KeyA) { direction -= Vec2::X; }
    if keys.pressed(KeyCode::KeyD) { direction += Vec2::X; }

    if direction != Vec2::ZERO {
        player.0.translation = Some(direction.normalize() * speed * time.delta_secs());
        let player_level_pos = level::world_to_level_pos(Vec2::new(player.1.translation.x, player.1.translation.y));
        label.0 = format!("{}, {}", player_level_pos.x, player_level_pos.y);
    }

    Ok(())
}