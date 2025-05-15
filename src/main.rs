use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use game::GamePlugin;
pub mod game;

pub mod player;
pub mod level;
pub mod mechanics;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        title: "Avys".to_string(),
                        resizable: false, 
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),

            GamePlugin
        ))
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
