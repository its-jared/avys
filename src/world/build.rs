use bevy::prelude::*;
use super::*;

pub fn build_world(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite {
            image: asset_server.load("textures/missing.png"),
            custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
            ..Default::default()
        },
    ));
}