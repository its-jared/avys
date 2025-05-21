use bevy::prelude::*;

pub mod gui;
pub mod player;
pub mod game;
pub mod rotate_to_cursor;
pub mod enemy;
pub mod level;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest()),
            game::GamePlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
