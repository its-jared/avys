use bevy::prelude::*;
use super::*;

pub fn build_world(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            commands.spawn((
                Transform::from_xyz(
                    (x * TILE_SIZE) as f32, 
                    (y * TILE_SIZE) as f32, 
                    0.0
                ),
                Sprite {
                    image: asset_server.load("grass.png"),
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                },
            ));
        }
    }
}