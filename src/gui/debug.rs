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

#[derive(Component)]
pub struct FacingLabel;

#[derive(Component)]
pub struct RunningLabel;

#[derive(Component)]
pub struct TimeLabel;

pub struct DebugGUIPlugin;
impl Plugin for DebugGUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, (
            update_fps_label,
            update_pos_label,
            update_delta_label,
            update_facing_label,
            update_running_label,
            update_time_label,
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
        font_smoothing: FontSmoothing::AntiAliased,
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

            parent.spawn((
                Text::new("Facing"),
                font.clone(), 
                FacingLabel,
            ));

            parent.spawn((
                Text::new("Running"),
                font.clone(), 
                RunningLabel,
            ));

            parent.spawn((
                Text::new("Time"),
                font.clone(), 
                TimeLabel,
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
    player_q: Query<&Player, Without<PlayerPosLabel>>,
) {
    let mut pos_label = pos_label_q.single_mut();
    let player = player_q.single();

    pos_label.0 
        = format!("POS: {}, {}",
        player.pos.x,
        player.pos.y);
}

fn update_delta_label(
    mut delta_label_q: Query<&mut Text, With<DeltaLabel>>,
    time: Res<Time>,
) {
    let mut delta_label = delta_label_q.single_mut();
    delta_label.0 = format!("Delta: {}", time.delta_secs());
}

fn update_facing_label(
    mut facing_label_q: Query<&mut Text, With<FacingLabel>>,
    player_q: Query<&Player, Without<FacingLabel>>,
) {
    let mut facing_label = facing_label_q.single_mut();
    let player = player_q.single();

    facing_label.0 
        = format!("Facing: {}", player.facing);
}

fn update_running_label(
    mut running_label_q: Query<&mut Text, With<RunningLabel>>,
    player_q: Query<&Player, Without<RunningLabel>>,
) {
    let mut running_label = running_label_q.single_mut();
    let running = player_q.single().running;

    running_label.0 
        = format!("Running: {}", running);
}

fn update_time_label(
    mut time_label_q: Query<&mut Text, With<TimeLabel>>,
    time: Res<Time>,
) {
    let mut time_label = time_label_q.single_mut();
    time_label.0 = format!("Time: {}", time.elapsed_secs().floor());
}