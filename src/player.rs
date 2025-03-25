use bevy::prelude::*;

use crate::personality::{Health, Mood};

#[derive(Component)]
pub struct Player(pub Vec3);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(FixedUpdate, player_movement);
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite {
            color: Color::srgb(1.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(16.0, 32.0)),
            ..Default::default()
        },
        Player(Vec3::new(1.0, 0.0, 1.0)),
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

fn player_movement(
    mut player_q: Query<(&mut Player, &mut Transform)>,
    mut camera_q: Query<(&Camera2d, &mut Transform), Without<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut move_dir = Vec3::ZERO;
    let mut player = player_q.single_mut();
    let mut cam_transform = camera_q.single_mut().1;

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

    if move_dir != Vec3::ZERO {
        let translation 
            = move_dir.normalize() * 100.0 * time.delta_secs();
        player.0.0 = move_dir;
        player.1.translation += translation;
    }

    cam_transform.translation = player.1.translation;
}