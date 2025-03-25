use bevy::prelude::*;
use crate::{gui::{debug::DebugGUIPlugin, hud::HUDPlugin}, player::PlayerPlugin};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            DebugGUIPlugin,
            HUDPlugin,
        ));
    }
}