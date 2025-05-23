use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod gui;
pub mod player;
pub mod game;
pub mod rotate_to_cursor;
pub mod enemy;
pub mod level;
pub mod ambiance;
pub mod config;

fn main() {
    App::new()
        .insert_resource(config::fetch_game_config())
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest()),

            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            //RapierDebugRenderPlugin::default(),
            
            game::GamePlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
