use bevy::prelude::*;
use crate::world::{global_to_world_pos, WORLD_SIZE};

use super::*;

fn move_player(
    mut player: (Mut<'_, Player>, Mut<'_, Transform>),
    mut cam_transform: Mut<'_, Transform>,
    move_dir: Vec3,
    detla_secs: f32,
) {
    let mut speed = WALKING_SPEED; 
    
    if player.0.running {
        speed = RUNNING_SPEED;
    }

    if move_dir != Vec3::ZERO {
        let translation 
            = move_dir * speed * detla_secs;

        // prevent player from leaving the world bounds. 
        // currently this doesn't work.
        let world_pos = global_to_world_pos(player.1.translation + translation);
        println!("{world_pos}");

        if world_pos.x > 0.0 || world_pos.x < WORLD_SIZE as f32 ||
            world_pos.y > 0.0 || world_pos.y < WORLD_SIZE as f32 { 
            player.1.translation += translation;
        }
    }

    cam_transform.translation = player.1.translation;
    player.0.pos = global_to_world_pos(player.1.translation);
}

/*
    === KEYBOARD ===
*/
pub fn keyboard_controls(
    mut player_q: Query<(&mut Player, &mut Transform)>,
    mut camera_q: Query<(&Camera2d, &mut Transform), Without<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut move_dir = Vec3::ZERO;
    let mut player = player_q.single_mut();
    let cam_transform = camera_q.single_mut().1;

    if keys.pressed(KeyCode::KeyW) {
        move_dir.y = 1.0;
        player.0.facing = Face::North;
    }
    if keys.pressed(KeyCode::KeyS) {
        move_dir.y = -1.0;
        player.0.facing = Face::South;
    }

    if keys.pressed(KeyCode::KeyA) {
        move_dir.x = -1.0;
        player.0.facing = Face::West;
    }
    if keys.pressed(KeyCode::KeyD) {
        move_dir.x = 1.0;
        player.0.facing = Face::East;
    }

    if keys.pressed(KeyCode::ShiftLeft) {
        //player.0.running = true;
    }

    move_player(player, cam_transform, move_dir, time.delta_secs());
}

/*
    === CONTROLLER ===
*/
pub fn gamepad_controls(
    mut player_q: Query<(&mut Player, &mut Transform)>,
    mut camera_q: Query<(&Camera2d, &mut Transform), Without<Player>>,
    gamepad_q:  Query<&Gamepad>,
) {
    for gamepad in &gamepad_q {   
        let mut move_dir = Vec3::ZERO;
        let mut player = player_q.single_mut();
        let cam_transform = camera_q.single_mut().1;

        // left joystick
        if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX)  {
            move_dir.x = left_stick_x;

            if left_stick_x < 0.0 { player.0.facing = Face::West; }
            else if left_stick_x > 0.0 { player.0.facing = Face::East; }
        }
        if let Some(left_stick_y) = gamepad.get(GamepadAxis::LeftStickY)  {
            move_dir.y = left_stick_y;

            if left_stick_y < 0.0 { player.0.facing = Face::South; }
            else if left_stick_y > 0.0 { player.0.facing = Face::North; }
        }

        // dpad 
        if gamepad.pressed(GamepadButton::DPadUp) {
            move_dir.y = 1.0;
            player.0.facing = Face::North;
        }
        if gamepad.pressed(GamepadButton::DPadDown) {
            move_dir.y = -1.0;
            player.0.facing = Face::South;
        }

        if gamepad.pressed(GamepadButton::DPadLeft) {
            move_dir.x = -1.0;
            player.0.facing = Face::West;
        }
        if gamepad.pressed(GamepadButton::DPadRight) {
            move_dir.x = 1.0;
            player.0.facing = Face::East;
        }

        // right joystick. 
        if let Some(right_stick_x) = gamepad.get(GamepadAxis::RightStickX)  {
            if right_stick_x < 0.0 { player.0.facing = Face::West; }
            else if right_stick_x > 0.0 { player.0.facing = Face::East; }
        }
        if let Some(right_stick_y) = gamepad.get(GamepadAxis::RightStickY)  {
            if right_stick_y < 0.0 { player.0.facing = Face::South; }
            else if right_stick_y > 0.0 { player.0.facing = Face::North; }
        }

        if gamepad.just_pressed(GamepadButton::South) {
            player.0.running = !player.0.running;
        }

        // this needs to be fixed to use delta time.
        move_player(player, cam_transform, move_dir, 0.01);
    }  
}