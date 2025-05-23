use std::collections::HashMap;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use block::Block;

use crate::config;

pub mod build;
pub mod block;
pub mod biome;
pub mod structure;

pub const TILE_SIZE: i32 = 64;
pub const LEVEL_SIZE: i32 = 100;

pub fn world_to_level_pos(wpos: Vec2) -> IVec2 {
    IVec2 {
        x: (wpos.x) as i32 / TILE_SIZE,
        y: (wpos.y) as i32 / TILE_SIZE,
    }
}

pub fn level_to_world_pos(lpos: IVec2, z: f32) -> Vec3 {
    Vec3 { 
        x: (lpos.x * TILE_SIZE) as f32, 
        y: (lpos.y * TILE_SIZE) as f32,
        z
    }
}

#[derive(Resource)]
pub struct Level {
    pub floor_layer: HashMap<IVec2, (usize, Entity)>,
    pub wall_layer: HashMap<IVec2, (usize, Entity)>,
    pub block_registery: Vec<Block>,
}

#[allow(dead_code)]
impl Level {
    fn new() -> Self {
        Self {
            floor_layer: HashMap::new(),
            wall_layer: HashMap::new(),
            block_registery: block::get_blocks(),
        }
    }

    pub fn set_block(
        &mut self, c: &mut Commands, a: &AssetServer,
        pos: IVec2, id: usize
    ) {
        if let Some(block) = self.block_registery.get(id) {
            let block_entity: Entity;

            if !block.solid {
                block_entity = c.spawn((
                    Transform::from_translation(level_to_world_pos(pos, 0.0))
                        .with_scale(Vec3::splat(4.0)),
                    Sprite::from_image(a.load(format!("textures/{}.png", block.texture_id))) 
                )).id();
            } else {
                block_entity = c.spawn((
                    Transform::from_translation(level_to_world_pos(pos, 0.0))
                        .with_scale(Vec3::splat(4.0)),
                    Sprite::from_image(a.load(format!("textures/{}.png", block.texture_id))),
                    Collider::cuboid(block.colider_size, block.colider_size)
                )).id();
            }

            match block.layer {
                block::BlockLayer::Floor => {
                    self.remove_block(c, pos, block::BlockLayer::Floor);
                    self.floor_layer.insert(pos, (id, block_entity));
                },
                block::BlockLayer::Wall => {
                    self.remove_block(c, pos, block::BlockLayer::Wall);
                    self.wall_layer.insert(pos, (id, block_entity));
                }
            }
        }
    }

    pub fn remove_block(
        &mut self, c: &mut Commands,
        pos: IVec2, layer: block::BlockLayer,
    ) -> bool {
        match layer {
            block::BlockLayer::Floor => {
                if self.floor_layer.contains_key(&pos) {
                    let block = self.floor_layer.get(&pos).unwrap().to_owned();
                    c.entity(block.1).despawn();
                    self.floor_layer.remove(&pos);

                    return true;
                }
            },
            block::BlockLayer::Wall => {
                if self.wall_layer.contains_key(&pos) {
                    let block = self.wall_layer.get(&pos).unwrap().to_owned();
                    c.entity(block.1).despawn();
                    self.wall_layer.remove(&pos);

                    return true;
                }
            }
        }

        false
    }

    pub fn get_block(
        &self, pos: IVec2, layer: block::BlockLayer
    ) -> Option<usize> {
        match layer {
            block::BlockLayer::Floor => {
                if self.floor_layer.contains_key(&pos) {
                    return Some(self.floor_layer.get(&pos).unwrap().0);
                }
            },
            block::BlockLayer::Wall => {
                if self.wall_layer.contains_key(&pos) {
                    return Some(self.wall_layer.get(&pos).unwrap().0);
                }
            }
        }

        None
    }

    pub fn print_blocks(&self) {
        println!("Floor");

        for floor in &self.floor_layer {
            println!("pos: {}, id: {}", floor.0, floor.1.0);
        }

        println!("Wall");

        for wall in &self.wall_layer {
            println!("pos: {}, id: {}", wall.0, wall.1.0);
        }

        println!("===");
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Level::new())
            .add_systems(Startup, build::build_level.run_if(resource_exists::<config::GameConfig>));
    }
}