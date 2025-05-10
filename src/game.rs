use bevy::prelude::*;
use crate::{
    level, 
    player,
    mechanics::*
};

pub struct GamePlugin; 

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(avys::AvysData {
                avys_destroyed: false,
                avys_effect: 0.1,
                avys_state: avys::AvysState::Dormant,
            })

            .add_plugins((
                player::PlayerPlugin,
                level::LevelBuilderPlugin,
            ));
    }
}