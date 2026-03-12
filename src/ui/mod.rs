use bevy::prelude::*;

pub mod chat;
pub mod hud;

pub const REGULAR_FONT: &'static str = "IosevkaCharon-Regular.ttf";
pub const BOLD_FONT: &'static str = "IosevkaCharon-Bold.ttf";

pub const FONT_SIZE: f32 = 15.0;

pub struct UIPlugin; 
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(chat::ChatPlugin)
            .add_systems(Startup, hud::build_hud)
            .add_systems(Update, hud::update_hud);
    }
}