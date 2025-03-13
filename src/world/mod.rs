use bevy::prelude::*;
use build::build_world;

pub mod build;

pub const CHUNK_SIZE: i32 = 10;
pub const CHUNK_HEIGHT: i32 = 100;

pub struct WorldPlugin; 
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_world);
    }
}