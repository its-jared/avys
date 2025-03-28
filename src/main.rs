use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ecs_tilemap::TilemapPlugin;
use game::GamePlugin;

pub mod game;
pub mod player;
pub mod world;
pub mod data;

pub mod gui;

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
                TilemapPlugin,
                GamePlugin
            )
        )
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.8)))
        .run();
}
