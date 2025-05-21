use bevy::prelude::*;
use crate::level;
use super::*;

pub fn handle_interaction(
    mut level: ResMut<level::Level>,
    mut c: Commands,
    a: Res<AssetServer>,
    buttons: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_cursor: Query<&mut Transform, With<PlayerCursor>>,
) {
    let window = q_window.single().unwrap();

    if let Some(cursor_pos) = window.cursor_position() {
        let (camera, camera_transform) = q_camera.single().unwrap();
        let mut player_cursor = q_cursor.single_mut().unwrap();
        let world_cursor_pos = camera.viewport_to_world_2d(camera_transform, cursor_pos).unwrap();
        let level_pos = level::world_to_level_pos(world_cursor_pos);

        player_cursor.translation = level::level_to_world_pos(level_pos, 2.0);

        if buttons.just_pressed(MouseButton::Left) {
            if level.remove_block(&mut c, level_pos) { level.set_block(&mut c, &a, level_pos, 1); }
        }        
        if buttons.just_pressed(MouseButton::Right) {
            level.print_blocks();
        }
    }
}