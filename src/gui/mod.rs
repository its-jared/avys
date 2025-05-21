use bevy::prelude::*;

pub mod debug_ui;
pub mod hotbar;

pub struct DebugUIPlugin;

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                debug_ui::setup,
                hotbar::setup,
            ));
    }
}