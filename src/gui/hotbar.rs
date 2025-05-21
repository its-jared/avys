use bevy::prelude::*;

pub fn setup(
    mut c: Commands,
    a: Res<AssetServer>,
) {
    c.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        children![
            (
                ImageNode::new(a.load("textures/hotbar.png")),
                Transform::from_scale(Vec3::splat(4.0))
            )
        ]
    ));
}