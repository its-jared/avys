use bevy::prelude::*;

use crate::player::PlayerCharacterPlugin;
use crate::world::WorldPlugin;

pub struct GamePlugin; 
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerCharacterPlugin,
            WorldPlugin
        ));
    }
}