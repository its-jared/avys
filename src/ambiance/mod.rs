use bevy::prelude::*;

pub mod birds;
pub mod music;

pub struct AmbiancePlugin;

impl Plugin for AmbiancePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                birds::setup_bird_sound,
                music::setup_music,
            ))
            .add_systems(Update, birds::manage_bird_sound);
    }
}
