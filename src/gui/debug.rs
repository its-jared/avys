/* note!
    fps and pos values are floored to prevent 
    overly long text labels. this flooring can 
    be turned off when the debug menu is 
    toggleable.
*/

use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, text::FontSmoothing};

use crate::player::Player;

#[derive(Component)]
pub struct FPSLabel;

#[derive(Component)]
pub struct PlayerPosLabel;

#[derive(Component)]
pub struct DeltaLabel;

pub struct DebugGUIPlugin;
impl Plugin for DebugGUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, (
            update_fps_label,
            update_pos_label,
            update_delta_label,
        ));
    }
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = TextFont {
        font: asset_server.load("m5x7.ttf"),
        font_size: 25.0,
        font_smoothing: FontSmoothing::None,
    };

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(25.0),
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
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
                Text::from("FPS"),
                font.clone(), 
                FPSLabel,
            ));

            parent.spawn((
                Text::new("Player POS"),
                font.clone(), 
                PlayerPosLabel,
            ));

            parent.spawn((
                Text::new("Delta"),
                font.clone(), 
                DeltaLabel,
            ));
        });
}

fn update_fps_label(
    mut fps_label_q: Query<&mut Text, With<FPSLabel>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let mut fps_label = fps_label_q.single_mut();

    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed()) {
        fps_label.0 = format!("FPS: {}", value.floor());
    } else {
        fps_label.0 = format!("FPS: N/A");
    }
}

fn update_pos_label(
    mut pos_label_q: Query<&mut Text, With<PlayerPosLabel>>,
    player_q: Query<(&Player, &Transform), Without<PlayerPosLabel>>,
) {
    let mut pos_label = pos_label_q.single_mut();
    let player_pos = player_q.single().1.translation;

    pos_label.0 
        = format!("POS: {}, {}",
        player_pos.x.floor(),
        player_pos.y.floor());
}

fn update_delta_label(
    mut delta_label_q: Query<&mut Text, With<DeltaLabel>>,
    time: Res<Time>,
) {
    let mut delta_label = delta_label_q.single_mut();
    delta_label.0 = format!("Delta: {}", time.delta_secs());
}