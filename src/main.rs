use bevy::prelude::*;

pub mod entity;
pub mod player;
pub mod ui;
pub mod animation;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Avys".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            player::PlayerPlugin,
            animation::AnimationPlugin,
            entity::EntityPlugin,
            ui::UIPlugin,
        ))
        .run();
}