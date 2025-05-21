use bevy::prelude::*;
use crate::{gui, level, player, rotate_to_cursor};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                gui::DebugUIPlugin,
                player::PlayerPlugin,
                level::LevelPlugin,
            ))
            
            .add_systems(Update, rotate_to_cursor::handle_rotations);
    }
}