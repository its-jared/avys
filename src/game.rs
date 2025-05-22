use bevy::prelude::*;
use crate::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                gui::DebugUIPlugin,
                player::PlayerPlugin,
                level::LevelPlugin,
                ambiance::AmbiancePlugin,
            ))
            
            .add_systems(Update, rotate_to_cursor::handle_rotations);
    }
}