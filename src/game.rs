use bevy::prelude::*;

use crate::world::WorldPlugin;

pub struct GamePlugin; 
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldPlugin);
    }
}