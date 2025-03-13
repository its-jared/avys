use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use game::GamePlugin;

pub mod game;
pub mod world;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            position: WindowPosition::Centered(MonitorSelection::Primary),
                            title: "Avys".to_string(),
                            resizable: false, 
                            ..default()
                        }),
                        ..default()
                    }),
                
                PlayerPlugin,
                GamePlugin,
            )
        )
        .insert_resource(ClearColor(Color::WHITE))
        .run();
}
