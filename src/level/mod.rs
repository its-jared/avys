use std::collections::HashMap;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use block::Block;

use crate::config;

pub mod build;
pub mod block;
pub mod biome;
pub mod structure;
pub mod infinite;
pub mod chunk;

pub const TILE_SIZE: i32 = 64;
pub const CHUNK_SIZE: i32 = 16;

pub fn level_to_chunk_pos(lpos: IVec2) -> IVec2 {
    ivec2(
        lpos.x / CHUNK_SIZE,
        lpos.y / CHUNK_SIZE,
    )
}

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
    pub block_registery: Vec<Block>,
    pub biome_registery: Vec<Box<dyn biome::Biome>>,
    pub chunk_registery: HashMap<IVec2, chunk::Chunk>,
}

#[allow(dead_code)]
impl Level {
    fn new() -> Self {        
        Self {
            block_registery: block::get_blocks(),
            biome_registery: biome::get_biomes(),
            chunk_registery: HashMap::new(),
        }
    }

    pub fn set_block(
    &mut self, c: &mut Commands, a: &AssetServer,
    pos: IVec2, id: usize
    ) {
        let chunk_pos = level_to_chunk_pos(pos);

        if let Some(chunk) = self.chunk_registery.get(&chunk_pos) {
            let mut nchunk = chunk.clone();
            nchunk.set_block(c, a, &self, chunk_pos, id);
        } else {
            self.build_chunk_at(c, a, pos);
        }
    }

    pub fn get_chunk(&self, pos: IVec2) -> Option<&chunk::Chunk> {
        self.chunk_registery.get(&pos)
    }

    pub fn build_chunk_at(
        &mut self, c: &mut Commands, a: &AssetServer,
<<<<<<< HEAD
        pos: IVec2) {
        let mut chunk = chunk::Chunk::new();
        
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let p = ivec2(x, y);
                chunk.set_block(c, a, &self, p, 1);
            }
        }

        self.chunk_registery.insert(pos, chunk);
=======
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
>>>>>>> parent of 09a6720 (desert)
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