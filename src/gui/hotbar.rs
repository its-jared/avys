use bevy::prelude::*;

#[derive(Component)]
pub struct BlockIndicator;

pub fn setup(
    mut c: Commands,
    a: Res<AssetServer>,
) {
    c.spawn((
        Node {
            width: Val::Px(64.0),
            height: Val::Px(64.0),
            ..default()
        },
        children![
            (
                ImageNode::new(a.load("textures/player.png")),
                BlockIndicator,
            )
        ]
    ));
}