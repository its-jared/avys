use bevy::prelude::*;

pub mod game;
use game::GamePlugin;

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
        .run();
}
