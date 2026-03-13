use bevy::prelude::*;

pub mod entity;
pub mod player;
pub mod ui;
pub mod animation;
pub mod splash_text;

fn main() {
    let splash = splash_text::get_splash_text()
        .expect("Error when getting splash text!");
    
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!("Avys - {}", splash).to_string(),
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