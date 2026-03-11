use bevy::prelude::*;

use crate::{entity::{Health, Stamina}, player::Player};

#[derive(Component)]
pub struct HUD;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct StaminaBar;

pub fn build_hud(
    mut c: Commands,
) {
    c.spawn(
    Node {
        width: percent(100),
        height: percent(100),
        flex_direction: FlexDirection::Column,
        column_gap: Val::Px(15.0),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn((
            HealthBar, 

            Text::new("Health will go here."),
            TextFont {
                font_size: 12.0,
                ..default()
            },
        ));

        parent.spawn((
            StaminaBar, 

            Text::new("Stamina will go here."),
            TextFont {
                font_size: 12.0,
                ..default()
            },
        ));
    });
}

pub fn update_hud(
    mut health_bar: Single<&mut Text, (With<HealthBar>, Without<StaminaBar>)>,
    mut stamina_bar: Single<&mut Text, (With<StaminaBar>, Without<HealthBar>)>,
    player_q: Query<(&Health, &Stamina), With<Player>>,
) {
    for (health, stamina) in player_q.iter() {
        health_bar.0 = format!("Health: {}/{}", health.value, health.max);
        stamina_bar.0 = format!("Stamina: {}/{}", stamina.value, stamina.max);
    }
}