use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use game::GamePlugin;

pub mod game;
pub mod player;

pub mod gui;
pub mod personality;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            position: WindowPosition::Centered(MonitorSelection::Primary),
                            title: "Avys".to_string(),
                            resizable: true, 
                            ..default()
                        }),
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest()),
                
                FrameTimeDiagnosticsPlugin::default(),

                GamePlugin
            )
        )
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
