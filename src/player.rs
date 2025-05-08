use bevy::prelude::*;

#[derive(Component)]
pub struct Player; 

fn setup_player(mut cmds: Commands) {
    cmds.spawn((
        Camera2d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
    ));
}

fn handle_movement(
    mut player_q: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) -> Result<(), BevyError> {
    let mut player = player_q.single_mut()?;
    let mut direction = Vec3::ZERO; 

    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        player.translation += direction * time.delta_secs() * 500.0;
    }

    Ok(())
}

pub struct PlayerPlugin; 

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player)
            .add_systems(Update, handle_movement);
    }
}