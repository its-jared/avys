use bevy::{prelude::*, text::FontSmoothing};

use crate::{personality::{Health, Mood}, player::Player};

#[derive(Component)]
pub struct PlayerHealthLabel;

#[derive(Component)]
pub struct PlayerMoodLabel;

pub struct HUDPlugin;
impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud);
        app.add_systems(Update, (
            update_health_label,
            update_mood_label
        ));
    }
}

fn setup_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = TextFont {
        font: asset_server.load("m5x7.ttf"),
        font_size: 25.0,
        font_smoothing: FontSmoothing::AntiAliased,
    };

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(25.0),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            margin: UiRect {
                left: Val::Px(12.0),
                right: Val::Px(12.0),
                top: Val::Px(12.0),
                bottom: Val::Px(12.0),
            },
            ..Default::default()
        })
        .insert(PickingBehavior::IGNORE)
        .with_children(|parent| {
            parent.spawn((
                Text::from("HP N/A"),
                font.clone(), 
                TextColor::from(Color::srgb(1.0, 0.0, 0.0)),
                PlayerHealthLabel,
            ));

            parent.spawn((
                Text::from("Mood N/A"),
                font.clone(), 
                TextColor::from(Color::srgb(0.0, 1.0, 0.0)),
                PlayerMoodLabel,
            ));
        });
}

fn update_health_label(
    mut health_label_q: Query<&mut Text, With<PlayerHealthLabel>>,
    player_q: Query<(&Player, &Health), Without<PlayerHealthLabel>>,
) {
    let mut health_label = health_label_q.single_mut();
    let player_health = player_q.single().1;

    health_label.0 = format!("{}/{}", player_health.val, player_health.max);
}

fn update_mood_label(
    mut mood_label_q: Query<&mut Text, With<PlayerMoodLabel>>,
    player_q: Query<(&Player, &Mood), Without<PlayerMoodLabel>>,
) {
    let mut mood_label = mood_label_q.single_mut();
    let player_mood = player_q.single().1;

    mood_label.0 = format!("{}", player_mood);
}