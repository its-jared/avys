use bevy::{prelude::*, window::WindowMode};
use text_io::read;

use crate::{game::GamePlugin, world::terrain_type::{CurrentTerrainType, TerrainTypes, collect_terrain_types}};

pub mod player;
pub mod world;
pub mod debug_panel;
pub mod game;

fn main() {
    println!("Enter terrain type ID:");
    let mut current_terrain_type: String = read!("{}\n");
    
    if current_terrain_type == "" { current_terrain_type = "archipelago".into(); }

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(TerrainTypes(collect_terrain_types()))
        .insert_resource(CurrentTerrainType(current_terrain_type))

        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Avys".into(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            GamePlugin
        ))

        .run();
}
