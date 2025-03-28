use bevy::prelude::*;
use crate::{data::DataPlugin, gui::{debug::DebugGUIPlugin, hud::HUDPlugin}, player::PlayerPlugin, world::WorldPlugin};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DataPlugin,
            PlayerPlugin,
            DebugGUIPlugin,
            HUDPlugin,
            WorldPlugin,
        ));
    }
}