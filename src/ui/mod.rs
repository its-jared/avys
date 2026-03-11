use bevy::prelude::*;

pub mod chat;
pub mod hud;

pub struct UIPlugin; 
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(chat::ChatPlugin)
            .add_systems(Startup, hud::build_hud)
            .add_systems(Update, hud::update_hud);
    }
}