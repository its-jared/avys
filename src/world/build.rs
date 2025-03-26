use bevy::prelude::*;

pub fn build_world(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite {
            image: asset_server.load("textures/missing.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 0.5,
            },
            custom_size: Some(Vec2::new(1000.0, 1000.0)),
            ..Default::default()
        },
    ));
}