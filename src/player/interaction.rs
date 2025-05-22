use bevy::{input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*};
use crate::level;
use super::*;

#[derive(Resource)]
pub struct ActiveItem(pub usize);

pub fn handle_interaction(
    mut level: ResMut<level::Level>,
    mut c: Commands,
    a: Res<AssetServer>,
    active_item: Res<ActiveItem>,
    buttons: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_cursor: Query<(&mut Transform, &mut Sprite), With<PlayerCursor>>,
) {
    let window = q_window.single().unwrap();

    if let Some(cursor_pos) = window.cursor_position() {
        let (camera, camera_transform) = q_camera.single().unwrap();
        let mut player_cursor = q_cursor.single_mut().unwrap();
        let world_cursor_pos = camera.viewport_to_world_2d(camera_transform, cursor_pos).unwrap();
        let level_pos = level::world_to_level_pos(world_cursor_pos);

        player_cursor.0.translation = level::level_to_world_pos(level_pos, 2.0);
        player_cursor.1.image = a.load(format!("textures/{}.png", level.block_registery.get(active_item.0).unwrap().texture_id));
        player_cursor.1.color = Color::srgba(1.0, 1.0, 1.0, 0.5);

        if buttons.pressed(MouseButton::Left) {
            level.remove_block(&mut c, level_pos, level::block::BlockLayer::Wall);
        }

        if buttons.pressed(MouseButton::Right) {
            level.set_block(&mut c, &a, level_pos, active_item.0);
        }
    }
}

#[allow(unused_assignments)]
pub fn change_selected_block(
    mut active_item: ResMut<ActiveItem>,
    mut evr_scroll: EventReader<MouseWheel>,
    buttons: Res<ButtonInput<KeyCode>>,
    level: Res<level::Level>,
) {
    let mut item = active_item.0 as i32;

    for ev in evr_scroll.read() {
        if ev.unit == MouseScrollUnit::Line { item += ev.y as i32; }
    }

    if buttons.just_pressed(KeyCode::Equal) { item += 1; }
    if buttons.just_pressed(KeyCode::Minus) { item -= 1; }

    if item != 0 {
        if item <= 0 { item = level.block_registery.len() as i32 - 1; }
        if item >= level.block_registery.len() as i32 { item = 1; }

        active_item.0 = item as usize;
    }
}