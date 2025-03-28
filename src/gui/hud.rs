use bevy::{prelude::*, text::FontSmoothing};

pub struct HUDPlugin;
impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud);
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
                Text::from("HUD to be implemented"),
                font.clone(), 
            ));
        });
}