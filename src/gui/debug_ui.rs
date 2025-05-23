use bevy::prelude::*;
use crate::config;

#[derive(Component)]
pub struct PlayerPosLabel;

pub fn setup(
    mut c: Commands,
    a: Res<AssetServer>,
    config: Res<config::GameConfig>,
) {
    let font = TextFont {
        font: a.load("Monocraft.ttc"),
        font_size: 24.0,
        ..default()
    };

    /*c.spawn((
        Text::new(format!("v{}.{}.{}", config.version.0, config.version.1, config.version.2)),
        font.clone(),
    ));*/

    c.spawn((
        Text::new("Player POS Here"),
        font.clone(),
        PlayerPosLabel,
    ));
}