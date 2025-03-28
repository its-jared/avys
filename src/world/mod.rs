use bevy::prelude::*;
use build::build_world;

use crate::data::BlockResource;

pub mod build;
pub mod world_commands;

pub const WORLD_SIZE: usize = 32;
pub const TILE_SIZE: usize = 32;

#[derive(Component)]
pub struct WorldTile(pub Vec2);

pub fn global_to_world_pos(pos: Vec3) -> Vec3 {
    Vec3::new(
        (pos.x / TILE_SIZE as f32).floor(),
        (pos.y / TILE_SIZE as f32).floor(),
        pos.z
    )
}

pub fn world_to_global_pos(pos: Vec3) -> Vec3 {
    Vec3::new(
        (pos.x * TILE_SIZE as f32).floor(),
        (pos.y * TILE_SIZE as f32).floor(),
        pos.z
    )
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, build_world);
    }
}