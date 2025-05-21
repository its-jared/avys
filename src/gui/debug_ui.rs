use bevy::prelude::*;

pub fn setup(
    mut c: Commands,
    a: Res<AssetServer>,
) {
    let font = TextFont {
        font: a.load("Monocraft.ttc"),
        font_size: 12.0,
        ..default()
    };

    c.spawn((
        Text::new("v0.0.1"),
        font.clone(),
    ));
}