use bevy::prelude::*;

#[derive(Component)]
pub struct RotateToCursor;

pub fn handle_rotations(
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_rotators: Query<(&mut Transform, &GlobalTransform), With<RotateToCursor>>,
) {
    let window = q_window.single().unwrap();

    if let Some(cursor_pos) = window.cursor_position() {
        let (camera, camera_transform) = q_camera.single().unwrap();

        let world_cursor_pos = camera.viewport_to_world_2d(camera_transform, cursor_pos).unwrap();
        for (mut transform, global_transform) in q_rotators.iter_mut() {
            let sprite_pos = global_transform.translation().truncate();
            let direction = world_cursor_pos - sprite_pos;
            let angle = direction.x.atan2(direction.y);

            transform.rotation = Quat::from_rotation_z(-angle);
        }
    }
}