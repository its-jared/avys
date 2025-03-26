use bevy::prelude::*;

use crate::{personality::{Health, Mood}};

pub const WALKING_SPEED: f32 = 100.0;
pub const RUNNING_SPEED: f32 = 150.0;

#[derive(Component)]
pub struct Player(pub Vec3);
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(FixedUpdate, (
            player_movement,
            gamepad_controls,
        ));
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 1.0),
        Sprite {
            color: Color::srgb(1.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(16.0, 32.0)),
            ..Default::default()
        },
        Player(Vec3::ZERO),
        Health {
            max: 20.0,
            val: 20.0
        },
        Mood::Happy,
    ));

    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(0.0, 0.0, 2.0),
    ));

    // this is just a demo square to ensure movement is working.
    commands.spawn((
        Transform::from_xyz(10.0, 10.0, 0.0),
        Sprite {
            image: asset_server.load("grass.png"),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..Default::default()
        },
    ));
}

fn move_player(
    mut player: (Mut<'_, Player>, Mut<'_, Transform>),
    mut cam_transform: Mut<'_, Transform>,
    move_dir: Vec3,
    detla_secs: f32,
    speed: f32,
) {
    if move_dir != Vec3::ZERO {
        let translation 
            = move_dir * speed * detla_secs;
        player.0.0 = move_dir;
        player.1.translation += translation;
    }

    cam_transform.translation = player.1.translation;
}

fn player_movement(
    mut player_q: Query<(&mut Player, &mut Transform)>,
    mut camera_q: Query<(&Camera2d, &mut Transform), Without<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut move_dir = Vec3::ZERO;
    let mut speed = WALKING_SPEED;
    let player = player_q.single_mut();
    let cam_transform = camera_q.single_mut().1;

    if keys.pressed(KeyCode::KeyW) {
        move_dir.y = 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        move_dir.y = -1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        move_dir.x = -1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        move_dir.x = 1.0;
    }

    if keys.pressed(KeyCode::ShiftLeft) {
        speed = RUNNING_SPEED;
    }

    move_player(player, cam_transform, move_dir, time.delta_secs(), speed);
}

fn gamepad_controls(
    mut player_q: Query<(&mut Player, &mut Transform)>,
    mut camera_q: Query<(&Camera2d, &mut Transform), Without<Player>>,
    gamepad_q:  Query<&Gamepad>,
) {
    for gamepad in &gamepad_q {   
        let mut move_dir = Vec3::ZERO;
        let mut speed = WALKING_SPEED;
        let player = player_q.single_mut();
        let cam_transform = camera_q.single_mut().1;

        // dpad 
        if gamepad.just_pressed(GamepadButton::DPadUp) {
            move_dir.y = 1.0;
        }
        if gamepad.just_pressed(GamepadButton::DPadDown) {
            move_dir.y = -1.0;
        }

        if gamepad.just_pressed(GamepadButton::DPadLeft) {
            move_dir.x = -1.0;
        }
        if gamepad.just_pressed(GamepadButton::DPadRight) {
            move_dir.x = 1.0;
        }

        // left joystick
        if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX)  {
            move_dir.x = left_stick_x;
        }
        if let Some(left_stick_y) = gamepad.get(GamepadAxis::LeftStickY)  {
            move_dir.y = left_stick_y;
        }

        if gamepad.just_pressed(GamepadButton::South) {
            speed = RUNNING_SPEED;
        }

        // this needs to be fixed to use delta time.
        move_player(player, cam_transform, move_dir, 0.01, speed);
    }  
}